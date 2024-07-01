use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(.+) would (.+) (\d+) happiness units by sitting next to (.+)\.$").unwrap();
}

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d13/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut scorings = HashMap::new();
    let mut subjects = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let scoring: Scoring = line.parse()?;
        subjects.insert(scoring.subject.clone());
        scorings.insert((scoring.subject, scoring.neighbour), scoring.change);
    }

    subjects.insert("You".into());

    let max_score = subjects
        .iter()
        .permutations(subjects.len())
        .map(|perm| {
            perm.iter()
                .circular_tuple_windows()
                .map(|(&left, &subject, &right)| {
                    let left = scorings
                        .get(&(subject.to_string(), left.to_string()))
                        .unwrap_or(&0);
                    let right = scorings
                        .get(&(subject.to_string(), right.to_string()))
                        .unwrap_or(&0);
                    left + right
                })
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("Max score: {}", max_score);

    Ok(())
}

#[derive(Debug)]
struct Scoring {
    subject: String,
    neighbour: String,
    change: i32,
}

impl FromStr for Scoring {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cap = RE
            .captures(s)
            .ok_or(anyhow::anyhow!("Unable to match {}", s))?;

        let change: i32 = match cap.get(2).unwrap().as_str() {
            "gain" => cap.get(3).unwrap().as_str().parse()?,
            "lose" => -cap.get(3).unwrap().as_str().parse()?,
            s => anyhow::bail!("Unknown value of {}", s),
        };

        Ok(Scoring {
            subject: cap.get(1).unwrap().as_str().into(),
            neighbour: cap.get(4).unwrap().as_str().into(),
            change,
        })
    }
}
