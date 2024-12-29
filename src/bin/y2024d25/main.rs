use anyhow::Result;
use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d25/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut block = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if !line.is_empty() {
            let chars: [char; 5] = line.chars().collect_vec().try_into().unwrap();
            block.push(chars);
            continue;
        }

        process_block(&mut block, &mut keys, &mut locks);
    }
    process_block(&mut block, &mut keys, &mut locks);


    let mut pair_count = 0;

    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5) {
                pair_count += 1;
            }
        }
    }

    println!("pair_count: {}", pair_count);

    Ok(())
}

fn process_block(block: &mut Vec<[char; 5]>, keys: &mut Vec<[u8; 5]>, locks: &mut Vec<[u8; 5]>) {
    assert_eq!(block.len(), 7);

    if block[0] == ['#', '#', '#', '#', '#'] {
        locks.push(count_pins(block));
    } else if block[6] == ['#', '#', '#', '#', '#'] {
        block.reverse();
        keys.push(count_pins(block));
    } else {
        unreachable!("invalid block");
    }

    block.clear();
}

fn count_pins(block: &Vec<[char; 5]>) -> [u8; 5] {
    let mut pins = [0; 5];

    for i in 0..5 {
        for j in 1..block.len() {
            if block[j][i] == '#' {
                pins[i] += 1;
            }
        }
    }

    pins
}


#[cfg(test)]
mod tests {
    

}
