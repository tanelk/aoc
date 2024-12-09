use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d08/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        max_x = max_x.max(y as i32);
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            max_y = max_y.max(x as i32);

            if c == '.' {
                continue;
            }

            antennas
                .entry(c)
                .or_insert(Vec::new())
                .push((x as i32, y as i32));
        }
    }

    //println!("{:?}", antennas);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, positions) in antennas.iter() {
        positions.iter().tuple_combinations().for_each(|(a, b)| {
            let x1 = 2 * a.0 - b.0;
            let y1 = 2 * a.1 - b.1;
            if x1 >= 0 && x1 <= max_x && y1 >= 0 && y1 <= max_y {
                antinodes.insert((x1, y1));
            }

            let x2 = 2 * b.0 - a.0;
            let y2 = 2 * b.1 - a.1;
            if x2 >= 0 && x2 <= max_x && y2 >= 0 && y2 <= max_y {
                antinodes.insert((x2, y2));
            }
        });
    }

    //println!("{:?}", antinodes);
    println!("{}", antinodes.len());

    // Part 2
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, positions) in antennas.iter() {
        positions.iter().tuple_combinations().for_each(|(a, b)| {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            let d = gcd(dx.abs(), dy.abs());
            let dx = dx / d;
            let dy = dy / d;

            for i in 0..max_x.max(max_y) {
                let x = b.0 + dx * i;
                let y = b.1 + dy * i;
                if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
                    antinodes.insert((x, y));
                }
                let x = b.0 - dx * i;
                let y = b.1 - dy * i;
                if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
                    antinodes.insert((x, y));
                }
            }
        });
    }

    println!("{}", antinodes.len());

    Ok(())
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
