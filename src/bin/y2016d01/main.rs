use std::env;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2016d01/input.txt");
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    let mut location = (0i32, 0i32);
    let mut heading = (1, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert(location);

    for instruction in contents.split(", ") {
        let (direction, steps) = instruction.split_at(1);
        let steps: i32 = steps.parse()?;

        match direction {
            "L" => {
                (heading.0, heading.1) = (-heading.1, heading.0);
            }
            "R" => {
                (heading.0, heading.1) = (heading.1, -heading.0);
            }
            _ => unreachable!(),
        }

        for _ in 0..steps {
            location.0 += heading.0;
            location.1 += heading.1;

            if !visited.insert(location) {
                println!(
                    "Repeated visitation: {:?}, distance {}",
                    location,
                    location.0.abs() + location.1.abs()
                )
            }
        }
    }

    println!("{location:?}");
    println!("{}", location.0.abs() + location.1.abs());

    Ok(())
}
