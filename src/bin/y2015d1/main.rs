use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d1/input.txt");
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    let mut floor = 0;
    for (i, c) in contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            println!("Basement at: {}", i + 1);
        }
    }
    println!("Floor: {}", floor);

    Ok(())
}
