use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use anyhow::Result;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2019d18/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let maze: Vec<Vec<Tile>> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            l.bytes()
                .map(|b| match b {
                    b'@' => Tile::Start,
                    b'#' => Tile::Wall,
                    b'.' => Tile::Floor,
                    b'a'..=b'z' => Tile::Key(b - b'a'),
                    b'A'..=b'Z' => Tile::Door(b - b'A'),
                    _ => unreachable!("{}", l),
                })
                .collect()
        })
        .collect();

    let start = maze
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, tile)| match tile {
                Tile::Start => Some((i, j)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let key_count = maze
        .iter()
        .flatten()
        .filter(|t| matches!(t, Tile::Key(_)))
        .count();

    let instant = Instant::now();
    part1(&maze, start, key_count);
    println!("Time elapsed is: {:?}", instant.elapsed());

    let instant = Instant::now();
    part2(&maze, start, key_count);
    println!("Time elapsed is: {:?}", instant.elapsed());

    Ok(())
}

fn part1(maze: &[Vec<Tile>], start: (usize, usize), key_count: usize) {
    // (pos, keys) pairs
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        pos: start,
        keys: 0,
        steps: 0,
    });

    while let Some(State { pos, keys, steps }) = queue.pop() {
        if keys.count_ones() == key_count as u32 {
            println!("Part 1: {}", steps);
            break;
        }

        if visited.contains(&(pos, keys)) {
            continue;
        }

        visited.insert((pos, keys));

        for (extra_steps, pos, keys) in reachable_keys(maze, pos, keys) {
            queue.push(State {
                pos,
                keys,
                steps: steps + extra_steps,
            });
        }
    }
}

fn part2(maze: &[Vec<Tile>], start: (usize, usize), key_count: usize) {
    let mut maze = maze.to_owned();
    let (i, j) = start;
    maze[i][j] = Tile::Wall;
    maze[i + 1][j] = Tile::Wall;
    maze[i - 1][j] = Tile::Wall;
    maze[i][j + 1] = Tile::Wall;
    maze[i][j - 1] = Tile::Wall;
    maze[i + 1][j + 1] = Tile::Start;
    maze[i + 1][j - 1] = Tile::Start;
    maze[i - 1][j + 1] = Tile::Start;
    maze[i - 1][j - 1] = Tile::Start;

    let pos = vec![
        (i + 1, j + 1),
        (i + 1, j - 1),
        (i - 1, j + 1),
        (i - 1, j - 1),
    ];

    // (pos, keys) pairs
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        pos,
        keys: 0,
        steps: 0,
    });

    while let Some(State { pos, keys, steps }) = queue.pop() {
        if keys.count_ones() == key_count as u32 {
            println!("Part 2: {}", steps);
            break;
        }

        for (i, partial_pos) in pos.clone().into_iter().enumerate() {
            if visited.contains(&(partial_pos, keys)) {
                continue;
            }

            visited.insert((partial_pos, keys));

            for (extra_steps, partial_pos, keys) in reachable_keys(&maze, partial_pos, keys) {
                let mut pos = pos.clone();
                pos[i] = partial_pos;

                queue.push(State {
                    pos,
                    keys,
                    steps: steps + extra_steps,
                });
            }
        }
    }
}

fn reachable_keys(
    maze: &[Vec<Tile>],
    pos: (usize, usize),
    keys: u32,
) -> Vec<(u32, (usize, usize), u32)> {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut queue = VecDeque::new();
    queue.push_back((pos, 0));

    let mut result = Vec::new();

    while let Some((pos, steps)) = queue.pop_front() {
        let (i, j) = pos;
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        match maze[i][j] {
            Tile::Wall => continue,
            Tile::Floor | Tile::Start => {
                queue.push_back(((i + 1, j), steps + 1));
                queue.push_back(((i - 1, j), steps + 1));
                queue.push_back(((i, j + 1), steps + 1));
                queue.push_back(((i, j - 1), steps + 1));
            }
            Tile::Key(k) => {
                if keys & (1 << k) == 0 {
                    result.push((steps, pos, keys | (1 << k)));
                } else {
                    queue.push_back(((i + 1, j), steps + 1));
                    queue.push_back(((i - 1, j), steps + 1));
                    queue.push_back(((i, j + 1), steps + 1));
                    queue.push_back(((i, j - 1), steps + 1));
                }
            }
            Tile::Door(d) => {
                if keys & (1 << d) != 0 {
                    queue.push_back(((i + 1, j), steps + 1));
                    queue.push_back(((i - 1, j), steps + 1));
                    queue.push_back(((i, j + 1), steps + 1));
                    queue.push_back(((i, j - 1), steps + 1));
                }
            }
        }
    }

    result
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Start,
    Wall,
    Floor,
    Key(u8),
    Door(u8),
}

#[derive(Debug, Copy, Clone)]
struct State<T> {
    pos: T,
    keys: u32,
    steps: u32,
}

impl<T> Ord for State<T> {
    /// This is a reversed ordering, so that the BinaryHeap will be a min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl<T> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for State<T> {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl<T> Eq for State<T> {}
