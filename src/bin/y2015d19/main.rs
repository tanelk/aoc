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

    // Part 2 - TODO this is not a general solution and can fail on some runs
    let reverse_replacements = replacements
        .iter()
        .flat_map(|(k, vs)| vs.iter().map(move |v| (v.clone(), k.clone())))
        .collect::<Vec<(String, String)>>();

    let mut steps = 0;
    let mut current_molecule = target_molecule.clone();
    while current_molecule != "e" {
        let mut found = false;
        for (k, v) in &reverse_replacements {
            if current_molecule.contains(k) {
                current_molecule = current_molecule.replacen(k, v, 1);
                steps += 1;
                found = true;
                break;
            }
        }

        if !found {
            panic!("No replacement found for {}", current_molecule);
        }
    }

    println!("Steps to make medicine: {}", steps);

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
