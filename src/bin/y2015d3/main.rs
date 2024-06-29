use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d3/input.txt");
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    println!("Visited {} houses", q1(&contents));
    println!("Visited {} houses", q2(&contents));

    Ok(())
}

fn q1(contents: &str) -> usize {
    let mut visited = std::collections::HashSet::new();

    let mut x = 0;
    let mut y = 0;

    visited.insert((x, y));
    for direction in contents.chars() {
        match direction {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Invalid direction: {}", direction)
        }
        visited.insert((x, y));
    }

    visited.len()
}


fn q2(contents: &str) -> usize {
    let mut visited = std::collections::HashSet::new();

    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut santa_moves = true;

    visited.insert((x1, y2));
    for direction in contents.chars() {
        let (x, y) = if santa_moves {
            (&mut x1, &mut y1)
        } else {
            (&mut x2, &mut y2)
        };

        santa_moves = !santa_moves;

        match direction {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => panic!("Invalid direction: {}", direction)
        }
        visited.insert((*x, *y));
    }

    visited.len()
}