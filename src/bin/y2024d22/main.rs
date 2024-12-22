use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d22/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut sequences: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut num: u64 = line.parse()?;
        let mut sequence = Vec::new();
        sequence.push(num);

        for _ in 0..2000 {
            num = next_number(num);
            sequence.push(num);
        }

        sequences.push(sequence);
    }

    let prices: Vec<Vec<i8>> = sequences
        .iter()
        .map(|sequence| sequence.iter().map(|num| (num % 10) as i8).collect())
        .collect();

    let price_changes: Vec<Vec<i8>> = prices
        .iter()
        .map(|prices| {
            let mut changes = Vec::new();
            for i in 1..prices.len() {
                changes.push(prices[i] - prices[i - 1]);
            }
            changes
        })
        .collect();

    let mut pattern_scores = HashMap::new();

    price_changes
        .iter()
        .zip(prices.iter())
        .for_each(|(changes, prices)| {
            let mut seen: HashSet<(&i8, &i8, &i8, &i8)> = HashSet::new();

            changes
                .iter()
                .tuple_windows()
                .enumerate()
                .filter(|(_, pattern)| seen.insert(*pattern))
                .for_each(|(i, pattern)| {
                    let score = pattern_scores.entry(pattern).or_insert(0);
                    *score += prices[i + 4] as i64;
                })
        });

    let max_score = pattern_scores.values().max().unwrap();
    println!("Max score: {}", max_score);

    Ok(())
}

fn next_number(num: u64) -> u64 {
    let base = 16777216;
    let num = (num ^ (num * 64)) % base;
    let num = (num ^ (num / 32)) % base;
    
    (num ^ (num * 2048)) % base
}

fn repeated_next_number(num: u64, n: usize) -> u64 {
    let mut num = num;
    for _ in 0..n {
        num = next_number(num);
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_num() {
        assert_eq!(next_number(0), 0);
        assert_eq!(next_number(123), 15887950);
        assert_eq!(next_number(15887950), 16495136);
    }

    #[test]
    fn test_repeated_next_number() {
        assert_eq!(repeated_next_number(0, 2000), 0);
        assert_eq!(repeated_next_number(1, 2000), 8685429);
    }
}
