use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d8/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut total_chars = 0;
    let mut total_symbols = 0;
    let mut total_encoded = 0;

    for line in reader.lines() {
        let line = line?;

        let chars = line.len();
        let symbols = count_symbols(&line);
        let encoded = encode(&line).as_str().len();

        total_chars += chars;
        total_symbols += symbols;
        total_encoded += encoded;
    }

    println!("Total chars: {}", total_chars);
    println!("Total symbols: {}", total_symbols);
    println!("Difference: {}", total_chars - total_symbols);

    println!("Total encoded: {}", total_encoded);
    println!("Difference: {}", total_encoded - total_chars);

    Ok(())
}

fn count_symbols(s: &str) -> usize {
    let mut chars = s.chars();
    let mut count = 0;

    while let Some(c) = chars.next() {
        match c {
            '"' => (),
            '\\' => {
                count += 1;
                match chars.next() {
                    Some('x') => {
                        chars.next();
                        chars.next();
                    }
                    Some(_) => (),
                    None => (),
                }
            }
            _ => count += 1,
        }
    }

    count
}

fn encode(s: &str) -> String {
    let mut encoded = String::with_capacity(2 * s.len());
    encoded.push('"');

    for c in s.chars() {
        match c {
            '"' => encoded.push_str("\\\""),
            '\\' => encoded.push_str("\\\\"),
            _ => encoded.push(c),
        }
    }

    encoded.push('"');
    encoded
}
