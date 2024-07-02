use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d14/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let race_duration = 2503;

    let mut max_distance = 0;
    let mut reindeers = Vec::new();
    let mut states = Vec::new();
    let mut scores = Vec::new();
    let mut locations = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let reindeer: Reindeer = line.parse()?;

        let cycle_duration = reindeer.flying_time + reindeer.rest_time;
        let cycles = race_duration / cycle_duration;
        let reminder = race_duration % cycle_duration;

        let distance =
            reindeer.speed * (reindeer.flying_time * cycles + reindeer.flying_time.min(reminder));
        max_distance = max_distance.max(distance);

        states.push(State::Fly(reindeer.flying_time));
        reindeers.push(reindeer);
        scores.push(0);
        locations.push(0);
    }

    for i in 1..=race_duration {
        for ((reindeer, state), location) in reindeers
            .iter()
            .zip(states.iter_mut())
            .zip(locations.iter_mut())
        {
            match state {
                State::Fly(until) => {
                    *location += reindeer.speed;

                    if *until == i {
                        *state = State::Rest(i + reindeer.rest_time);
                    }
                }
                State::Rest(until) => {
                    if *until == i {
                        *state = State::Fly(i + reindeer.flying_time);
                    }
                }
            }
        }

        let max_loc = *locations.iter().max().unwrap();
        for (location, score) in locations.iter().zip(scores.iter_mut()) {
            if *location == max_loc {
                *score += 1;
            }
        }
    }

    println!("Max distance: {}", max_distance);
    println!("Max score: {:?}", scores.iter().max());
    // Should be same as first
    println!("Max location: {:?}", locations.iter().max());

    Ok(())
}

enum State {
    Fly(i32),
    Rest(i32),
}

#[derive(Debug)]
struct Reindeer {
    speed: i32,
    flying_time: i32,
    rest_time: i32,
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$").unwrap();
        }

        let cap = RE
            .captures(s)
            .ok_or(anyhow::anyhow!("Unable to match {}", s))?;

        let speed = cap.get(2).unwrap().as_str().parse()?;
        let flying_time = cap.get(3).unwrap().as_str().parse()?;
        let rest_time = cap.get(4).unwrap().as_str().parse()?;

        Ok(Reindeer {
            speed,
            flying_time,
            rest_time,
        })
    }
}
