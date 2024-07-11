use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use itertools::{iproduct, Itertools};

type Coord = (i32, i32, i32);

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2018d23/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let re = regex::Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$")?;
    let mut bots = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let caps = re
            .captures(&line)
            .ok_or(anyhow::anyhow!("Invalid input: {}", line))?;
        let x = caps[1].parse::<i32>()?;
        let y = caps[2].parse::<i32>()?;
        let z = caps[3].parse::<i32>()?;
        let r = caps[4].parse::<i32>()?;

        bots.push((x, y, z, r));
    }

    let max_bot = bots.iter().max_by_key(|b| b.3).unwrap();
    let in_range = bots
        .iter()
        .filter(|b| {
            (max_bot.0 - b.0).abs() + (max_bot.1 - b.1).abs() + (max_bot.2 - b.2).abs() <= max_bot.3
        })
        .count();

    println!("Part 1: {}", in_range);

    // Determine max area
    // Split into 8 sub-cubes
    // Calc max intersections for sub-cubes
    // Recurse into the sub-cubes in the max intersection order
    // Halt early

    let max_cube_side = bots
        .iter()
        .map(|b| {
            b.0.unsigned_abs()
                .max(b.1.unsigned_abs())
                .max(b.2.unsigned_abs())
        })
        .max()
        .unwrap()
        .next_power_of_two() as i32;

    println!(
        "Max cube side: {}, (2**{})",
        max_cube_side,
        max_cube_side.ilog2()
    );

    let (loc, intersections) = search(
        &bots,
        (-max_cube_side, -max_cube_side, -max_cube_side),
        2 * max_cube_side,
        0,
    );

    let loc = loc.unwrap();

    println!("{:?} {}", loc, intersections);
    println!("Part 2: {:?}", manhattan_distance(loc));

    Ok(())
}

fn search(
    bots: &[(i32, i32, i32, i32)],
    cube: Coord,
    cube_side: i32,
    mut most_intersection_seen: usize,
) -> (Option<Coord>, usize) {
    if cube_side == 1 {
        return (Some(cube), count_intersections(bots, cube, 1));
    }

    let (x, y, z) = cube;
    let cube_side_half = cube_side / 2;

    let mut best_spot = None;

    iproduct![
        [x, x + cube_side_half],
        [y, y + cube_side_half],
        [z, z + cube_side_half],
    ]
    .map(|cube| {
        let upper_bound = count_intersections(bots, cube, cube_side_half);
        (cube, upper_bound)
    })
    // Ones with more intersections are more likely to contain the best point
    .sorted_by_key(|(_, max_intersections)| *max_intersections)
    .rev()
    .for_each(|(cube, max_intersections)| {
        if max_intersections <= most_intersection_seen {
            return;
        }

        let (spot, intersections) = search(bots, cube, cube_side_half, most_intersection_seen);
        (best_spot, most_intersection_seen) =
            match (intersections.cmp(&most_intersection_seen), spot, best_spot) {
                (Ordering::Greater, Some(spot), _) => (Some(spot), intersections),
                (Ordering::Equal, Some(spot), Some(best_spot)) => {
                    if manhattan_distance(spot) < manhattan_distance(best_spot) {
                        (Some(spot), intersections)
                    } else {
                        (Some(best_spot), most_intersection_seen)
                    }
                }
                _ => (best_spot, most_intersection_seen),
            }
    });

    (best_spot, most_intersection_seen)
}

/// Count the number of intersections in the cube defined by the given parameters:
/// - x, y, z: The coordinates of the cube's bottom-left corner
/// - cube_side: The length of the cube's side
fn count_intersections(bots: &[(i32, i32, i32, i32)], cube: Coord, cube_side: i32) -> usize {
    debug_assert!(cube_side > 0);

    let (x, y, z) = cube;

    // Simpler calculation if we reduce the cube by 1
    let cube_side = cube_side - 1;

    bots.iter()
        .filter(|b| {
            let nearest_x = b.0.min(x + cube_side).max(x);
            let nearest_y = b.1.min(y + cube_side).max(y);
            let nearest_z = b.2.min(z + cube_side).max(z);

            manhattan_distance((nearest_x - b.0, nearest_y - b.1, nearest_z - b.2)) <= b.3
        })
        .count()
}

fn manhattan_distance(c: Coord) -> i32 {
    c.0.abs() + c.1.abs() + c.2.abs()
}

#[cfg(test)]
mod tests {}
