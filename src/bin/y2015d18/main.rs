use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d18/input.txt");
    let reader = BufReader::new(File::open(path)?);

    const N: usize = 100;
    let mut lights = [[false; N]; N];
    let mut neighbours = [[0u8; N]; N];

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        for (j, char) in line.chars().enumerate() {
            match char {
                '.' => lights[i][j] = false,
                '#' => lights[i][j] = true,
                _ => unreachable!(),
            }
        }
    }

    lights[0][0] = true;
    lights[0][N - 1] = true;
    lights[N - 1][0] = true;
    lights[N - 1][N - 1] = true;

    for _ in 0..100 {
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            for j in 0..N {
                neighbours[i][j] = 0;

                let is = i.saturating_sub(1).max(0)..=(i + 1).min(N - 1);
                let js = j.saturating_sub(1)..=(j + 1).min(N - 1);

                for (i2, j2) in is.cartesian_product(js) {
                    if (i != i2 || j != j2) && lights[i2][j2] {
                        neighbours[i][j] += 1;
                    }
                }
            }
        }

        for i in 0..N {
            for j in 0..N {
                if lights[i][j] {
                    lights[i][j] = neighbours[i][j] == 2 || neighbours[i][j] == 3
                } else {
                    lights[i][j] = neighbours[i][j] == 3
                }
            }
        }

        lights[0][0] = true;
        lights[0][N - 1] = true;
        lights[N - 1][0] = true;
        lights[N - 1][N - 1] = true;

        let c = lights.iter().flat_map(|&s| s).filter(|&e| e).count();
        println!("{}", c);
    }

    Ok(())
}
