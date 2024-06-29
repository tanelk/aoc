use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d5/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut total = 0;
    let mut total2 = 0;

    for line in reader.lines() {
        let line = line?;

        if is_nice(&line) {
            total += 1;
        }

        if is_nice2(&line) {
            total2 += 1;
        }
    }

    println!("{}", total);
    println!("{}", total2);

    Ok(())
}

fn is_nice(s: &str) -> bool {
    let enough_vowels = s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3;
    let has_double = s.chars().tuple_windows().any(|(a, b)| a == b);
    let has_bad = ["ab", "cd", "pq", "xy"].iter().any(|p| s.contains(p));

    enough_vowels && has_double && !has_bad
}

fn is_nice2(s: &str) -> bool {
    let has_double_pair = s.chars()
        .tuple_windows()
        .enumerate()
        .any(|(i, (a, b))| s.chars().tuple_windows().skip(i+2).any(|(a2, b2)| a == a2 && b == b2));

    let repeats_w_gap = s.chars().tuple_windows().any(|(a, _, b)| a == b);

    has_double_pair && repeats_w_gap
}