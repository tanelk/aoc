use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d9/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut places = std::collections::HashSet::new();
    let mut distances = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line?;

        let parts = line.split_whitespace().collect_vec();

        let src = parts[0];
        let dest = parts[2];
        let dist = parts[4].parse::<i32>()?;

        places.insert(src.to_string());
        places.insert(dest.to_string());
        distances.insert((src.to_string(), dest.to_string()), dist);
        distances.insert((dest.to_string(), src.to_string()), dist);
    }

    let places = places.into_iter().collect_vec();

    let minmax = places
        .iter()
        .permutations(places.len())
        .map(|perm| {
            let mut dist = 0;
            for i in 0..perm.len() - 1 {
                dist += distances[&(perm[i].to_string(), perm[i + 1].to_string())];
            }

            (dist, perm)
        })
        .minmax();

    if let itertools::MinMaxResult::MinMax((min, _), (max, _)) = minmax {
        println!("Min: {}", min);
        println!("Max: {}", max);
    }

    Ok(())
}
