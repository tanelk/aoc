use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d19/input.txt");
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let re = Regex::new(r"^(.+) => (.+)$").unwrap();

    let mut replacements = HashMap::new();

    for line in lines.by_ref() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let captures = re
            .captures(&line)
            .ok_or(anyhow::anyhow!("Unable to match {}", line))?;

        replacements
            .entry(captures[1].to_string())
            .or_insert_with(Vec::new)
            .push(captures[2].to_string());
    }

    let target_molecule = lines.next().unwrap()?;
    let mut new_molecules = HashSet::new();

    run_replacement_step(&target_molecule, &replacements, &mut new_molecules);

    println!("Calibration value: {}", new_molecules.len());

    // Brute force solution of calling the replacement step until we reach the target molecule is too slow

    Ok(())
}

fn run_replacement_step(
    starting_molecule: &str,
    replacements: &HashMap<String, Vec<String>>,
    new_molecules: &mut HashSet<String>,
) {
    for (k, vs) in replacements {
        let klen = k.chars().count();
        for (index, _) in starting_molecule.match_indices(k) {
            for v in vs {
                let mut clone = starting_molecule.to_owned();
                clone.replace_range(index..index + klen, v);

                new_molecules.insert(clone);
            }
        }
    }
}
