use anyhow::Result;
use itertools::Itertools;
use std::fmt::Write;

fn main() -> Result<()> {
    let input = "1113222113";

    let mut result = input.to_string();
    for _ in 0..40 {
        result = look_and_say(&result)?;
    }

    println!("Part 1: {}", result.len());

    for _ in 0..10 {
        result = look_and_say(&result)?;
    }

    println!("Part 2: {}", result.len());

    Ok(())
}

fn look_and_say(input: &str) -> Result<String> {
    let mut buffer = String::with_capacity(2 * input.len());
    for (c, chunk) in &input.chars().chunk_by(|c| *c) {
        write!(buffer, "{}{}", chunk.count(), c)?
    }
    Ok(buffer)
}
