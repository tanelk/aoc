use anyhow::Result;
use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d03/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract())  {
            let a: usize = a.parse()?;
            let b: usize = b.parse()?;
            result += a * b;
        }
    }

    println!("Result: {}", result);

    Ok(())
}
