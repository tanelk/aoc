use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d01/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;

        line.split_whitespace().for_each(|s| {
            let n = i32::from_str(s).unwrap();
            numbers.push(n);
        });
    }

    let mut left: Vec<&i32> = numbers.iter().skip(0).step_by(2).collect();
    let mut right: Vec<&i32> = numbers.iter().skip(1).step_by(2).collect();
    left.sort();
    right.sort();

    let res: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum();

    println!("{}", res);

    let counts = right.iter().counts();
    let res: i32 = left
        .iter()
        .map(|n| {
            let count = counts.get(n).unwrap_or(&0);
            *n * (*count as i32)
        })
        .sum();

    println!("{}", res);

    Ok(())
}
