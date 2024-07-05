use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d24/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let weights: Result<Vec<i32>> = reader
        .lines()
        .map(|l| {
            l.map_err(Into::into)
                .and_then(|s| s.parse().map_err(Into::into))
        })
        .collect();

    let weights = weights?;

    let total: i32 = weights.iter().sum();
    assert_eq!(total % 3, 0);
    assert_eq!(total % 4, 0);

    let target_weight = total / 4;

    let res = find_best_subset(&weights, target_weight).unwrap();
    let prod: i64 = res.iter().map(|&i| i as i64).product();

    println!("{prod}");

    Ok(())
}

fn find_best_subset(candidates: &[i32], target_weight: i32) -> Option<Vec<i32>> {
    // Must exactly hit the target weight
    if target_weight == 0 {
        Some(Vec::new())
    } else if target_weight < 0 {
        None
    } else if let Some((candidate, rest)) = candidates.split_first() {
        let when_skipped = find_best_subset(rest, target_weight);

        let mut when_picked = find_best_subset(rest, target_weight - candidate);
        if let Some(ref mut v) = when_picked {
            v.push(*candidate);
        }

        match (when_skipped, when_picked) {
            (Some(a), Some(b)) => {
                // prefer smaller subsets and among them the ones with the smallest product.
                let a_len = a.len();
                let b_len = b.len();

                match a_len.cmp(&b_len) {
                    Ordering::Less => Some(a),
                    Ordering::Greater => Some(b),
                    Ordering::Equal => {
                        // Upcast to avoid overflow
                        let a_prod: i64 = a.iter().map(|&i| i as i64).product();
                        let b_prod: i64 = b.iter().map(|&i| i as i64).product();

                        if a_prod < b_prod {
                            Some(a)
                        } else {
                            Some(b)
                        }
                    }
                }
            }
            (a, b) => a.or(b),
        }
    } else {
        None
    }
}
