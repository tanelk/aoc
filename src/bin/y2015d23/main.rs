use anyhow::{bail, Result};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d23/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        instructions.push(line.parse()?)
    }

    let mut loc: i32 = 0;
    let mut registers: [u64; 2] = [1, 0];

    loop {
        match instructions.get(loc as usize) {
            None => break,
            Some(Instruction::Hlf(r)) => {
                registers[(r - b'a') as usize] /= 2;
                loc += 1;
            }
            Some(Instruction::Tpl(r)) => {
                registers[(r - b'a') as usize] *= 3;
                loc += 1;
            }
            Some(Instruction::Inc(r)) => {
                registers[(r - b'a') as usize] += 1;
                loc += 1;
            }
            Some(Instruction::Jmp(offset)) => {
                loc += offset;
            }
            Some(Instruction::Jie(r, offset)) => {
                if registers[(r - b'a') as usize] % 2 == 0 {
                    loc += offset;
                } else {
                    loc += 1;
                }
            }
            Some(Instruction::Jio(r, offset)) => {
                if registers[(r - b'a') as usize] == 1 {
                    loc += offset;
                } else {
                    loc += 1;
                }
            }
        }
    }

    println!("Registers: {:?}", registers);

    Ok(())
}

enum Instruction {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(i32),
    Jie(u8, i32),
    Jio(u8, i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let ascii = s.as_bytes();

        let i = match &ascii[..3] {
            b"hlf" => {
                let r = ascii[4];
                Instruction::Hlf(r)
            }
            b"tpl" => {
                let r = ascii[4];
                Instruction::Tpl(r)
            }
            b"inc" => {
                let r = ascii[4];
                Instruction::Inc(r)
            }
            b"jmp" => {
                let offset = std::str::from_utf8(&ascii[4..])?.parse()?;
                Instruction::Jmp(offset)
            }
            b"jie" => {
                let r = ascii[4];
                let offset = std::str::from_utf8(&ascii[7..])?.parse()?;
                Instruction::Jie(r, offset)
            }
            b"jio" => {
                let r = ascii[4];
                let offset = std::str::from_utf8(&ascii[7..])?.parse()?;
                Instruction::Jio(r, offset)
            }
            _ => bail!("Unknown value of {}", s),
        };

        Ok(i)
    }
}
