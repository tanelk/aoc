use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Pos = (usize, usize);
type Maze = Vec<Vec<Tile>>;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2024d20/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let maze: Maze = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.bytes()
                .map(|b| match b {
                    b'S' => Tile::Start,
                    b'E' => Tile::End,
                    b'#' => Tile::Wall,
                    b'.' => Tile::Floor,
                    _ => unreachable!("{}", l),
                })
                .collect()
        })
        .collect();

    let start = extract_pos(Tile::Start, &maze);
    let distances_from_start = distances_from(start, &maze);
    println!("Start: {:?}", start);

    let end = extract_pos(Tile::End, &maze);
    let distances_from_end = distances_from(end, &maze);
    println!("End: {:?}", end);

    let start_to_end = distances_from_start[&end];
    let end_to_start = distances_from_end[&start];
    assert_eq!(start_to_end, end_to_start);

    println!("Start to end: {}", start_to_end);

    let mut good_cheats = HashSet::new();

    distances_from_start
        .iter()
        .for_each(|(pos, distance_from_start)| {
            for neighbor in neighbors(*pos, &maze) {
                if maze[neighbor.0][neighbor.1] != Tile::Wall {
                    continue;
                }

                for neighbor2 in neighbors(neighbor, &maze) {
                    if maze[neighbor2.0][neighbor2.1] == Tile::Wall {
                        continue;
                    }

                    let distance_from_end = distances_from_end[&neighbor2];

                    let new_start_to_end = distance_from_start + 2 + distance_from_end;

                    if new_start_to_end >= start_to_end {
                        continue;
                    }

                    let saved = start_to_end - new_start_to_end;

                    if saved >= 100 {
                        good_cheats.insert((pos, neighbor2));
                    }
                }
            }
        });

    println!("Good cheats: {}", good_cheats.len());

    Ok(())
}

fn extract_pos(search_tile: Tile, maze: &Maze) -> Pos {
    maze.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, tile)| {
                if *tile == search_tile {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .next()
        .unwrap()
}

fn distances_from(start: Pos, maze: &Maze) -> HashMap<Pos, usize> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        let dist = distances[&pos];
        for neighbor in neighbors(pos, maze) {
            if maze[neighbor.0][neighbor.1] == Tile::Wall {
                continue;
            }

            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

fn neighbors((i, j): Pos, maze: &Maze) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if i < maze.len() - 1 {
        neighbors.push((i + 1, j));
    }
    if j < maze[i].len() - 1 {
        neighbors.push((i, j + 1));
    }
    neighbors
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    End,
    Wall,
    Floor,
}
