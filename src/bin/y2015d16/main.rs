use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d16/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let re = Regex::new(r"^Sue (.+): (.+): (\d+), (.+): (\d+), (.+): (\d+)$").unwrap();

    let mut measurements = std::collections::HashMap::new();
    measurements.insert("children", Amount::Exact(3));
    measurements.insert("cats", Amount::More(7));
    measurements.insert("samoyeds", Amount::Exact(2));
    measurements.insert("pomeranians", Amount::Less(3));
    measurements.insert("akitas", Amount::Exact(0));
    measurements.insert("vizslas", Amount::Exact(0));
    measurements.insert("goldfish", Amount::Less(5));
    measurements.insert("trees", Amount::More(3));
    measurements.insert("cars", Amount::Exact(2));
    measurements.insert("perfumes", Amount::Exact(1));

    'outer: for line in reader.lines() {
        let line = line?;
        let cap = re
            .captures(&line)
            .ok_or(anyhow::anyhow!("Unable to match {}", line))?;

        let number = &cap[1];
        for i in (2..=6).step_by(2) {
            let compound = &cap[i];
            let compound_amount = &cap[i + 1].parse::<i32>()?;

            let matches = match measurements[compound] {
                Amount::Exact(v) => *compound_amount == v,
                Amount::Less(v) => *compound_amount < v,
                Amount::More(v) => *compound_amount > v,
            };

            if !matches {
                continue 'outer;
            }
        }

        println!("Sue {} matches", number)
    }

    Ok(())
}

enum Amount {
    Exact(i32),
    Less(i32),
    More(i32),
}
