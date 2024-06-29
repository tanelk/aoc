use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Error};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d6/input.txt");
    let reader = BufReader::new(File::open(path)?);

    const N: usize = 1000;
    let mut lights = [[false; N]; N];
    let mut brightness = [[0u8; N]; N];

    for line in reader.lines() {
        let line = line?;
        let instruction: Instruction = line.parse()?;

        for x in instruction.xrange.clone() {
            for y in instruction.yrange.clone() {
                match instruction.operation {
                    Operation::TurnOn => {
                        lights[x][y] = true;
                        brightness[x][y] += 1;
                    }
                    Operation::TurnOff => {
                        lights[x][y] = false;
                        brightness[x][y] = brightness[x][y].saturating_sub(1);
                    }
                    Operation::Toggle => {
                        lights[x][y] = !lights[x][y];
                        brightness[x][y] += 2;
                    }
                }
            }
        }
    }

    let total_lights = lights.iter().flatten().filter(|&&light| light).count();
    let total_brightness: u32 = brightness.iter().flatten().map(|&b| b as u32).sum();
    println!("Total lights on: {}", total_lights);
    println!("Total brightness: {}", total_brightness);

    Ok(())
}

#[derive(Debug)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    xrange: RangeInclusive<usize>,
    yrange: RangeInclusive<usize>,
}

impl FromStr for Instruction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let operation = match parts.next() {
            Some("turn") => match parts.next() {
                Some("on") => Operation::TurnOn,
                Some("off") => Operation::TurnOff,
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid operation",
                    ))
                }
            },
            Some("toggle") => Operation::Toggle,
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid operation",
                ))
            }
        };

        let (xstart, ystart) = match parts.next() {
            Some(starts) => {
                let mut starts = starts.split(',');
                let xstart = starts.next().unwrap().parse().unwrap();
                let ystart = starts.next().unwrap().parse().unwrap();
                (xstart, ystart)
            }
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid starts",
                ))
            }
        };

        if parts.next() != Some("through") {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing 'through' keyword",
            ));
        }

        let (xend, yend) = match parts.next() {
            Some(ends) => {
                let mut ends = ends.split(',');
                let xend = ends.next().unwrap().parse().unwrap();
                let yend = ends.next().unwrap().parse().unwrap();
                (xend, yend)
            }
            _ => return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid ends")),
        };

        Ok(Instruction {
            operation,
            xrange: xstart..=xend,
            yrange: ystart..=yend,
        })
    }
}
