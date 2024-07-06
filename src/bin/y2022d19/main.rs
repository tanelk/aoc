use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use crate::Resource::{Clay, Geode, Obsidian, Ore};
use anyhow::Result;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let max_time = match args.get(1) {
        None => 24,
        Some(arg) => arg.parse::<i32>()?,
    };

    println!("Number of minutes to simulate: {max_time}");

    let path = env::current_dir()?.join("src/bin/y2022d19/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        blueprints.push(line.parse()?)
    }

    let mut total = 0;

    let results: Vec<_> = blueprints
        .par_iter()
        .map(|bp| {
            let max_geodes = maximize_geodes(max_time, bp);
            println!("{} geodes for {:?}", max_geodes, bp.id);
            (bp, max_geodes)
        })
        .collect();

    for (bp, max_geodes) in results {
        total += bp.id * max_geodes;
    }

    println!("Total quality: {total}");

    Ok(())
}

fn maximize_geodes(time_remaining: i32, blueprint: &Blueprint) -> i32 {
    let mut robots = HashMap::new();
    robots.insert(Ore, 1);
    let mut resources = HashMap::new();
    resources.insert(Geode, 0);

    let mut max_costs = HashMap::new();
    max_costs.insert(Ore, blueprint.robots.iter().map(|r| r.cost_ore).max().unwrap());
    max_costs.insert(Clay, blueprint.robots.iter().map(|r| r.cost_clay).max().unwrap());
    max_costs.insert(Obsidian, blueprint.robots.iter().map(|r| r.cost_obsidian).max().unwrap());
    max_costs.insert(Geode, i32::MAX);

    run_iteration(
        time_remaining,
        blueprint,
        &robots,
        &resources,
        HashSet::new(),
        -1,
        &max_costs,
    )
}

fn run_iteration(
    time_remaining: i32,
    blueprint: &Blueprint,
    robots: &HashMap<Resource, i32>,
    resources: &HashMap<Resource, i32>,
    // Optimization heuristics
    mut skipped: HashSet<Resource>,
    max_geodes_so_far: i32,
    max_costs: &HashMap<Resource, i32>
) -> i32 {
    if time_remaining <= 0 {
        return *resources.get(&Geode).unwrap_or(&0);
    }

    // Branch pruning heuristics:
    // 1) Keep track of best result seen so far
    // 2) There is no reason to skip, if you can build all robots
    // 3) If you skipped robot X on prev turn, then you should skip it this turn

    let upper_limit = {
        // Buy one geode robot each minute
        let current = *robots.get(&Geode).unwrap_or(&0);
        (current + current + time_remaining) * (1 + time_remaining) / 2
    };
    if upper_limit <= max_geodes_so_far {
        return -1;
    }

    let mut max_geodes = 0;
    let mut consider_skipping = false;

    let mut resources_after = resources.clone();
    for (r, c) in robots {
        *resources_after.entry(*r).or_insert(0) += c;
    }

    // Build a robot
    for robot in &blueprint.robots {
        // can build is based on resources at the start, not at the end of the turn
        let can_build = *resources.get(&Ore).unwrap_or(&0) >= robot.cost_ore
            && *resources.get(&Clay).unwrap_or(&0) >= robot.cost_clay
            && *resources.get(&Obsidian).unwrap_or(&0) >= robot.cost_obsidian;

        let could_build_after_skipping = ((robot.cost_clay == 0)
            || *robots.get(&Clay).unwrap_or(&0) > 0)
            && ((robot.cost_obsidian == 0) || *robots.get(&Obsidian).unwrap_or(&0) > 0);

        consider_skipping |= !can_build && could_build_after_skipping;

        if !can_build {
            continue;
        }

        // Do we have enough of this robot?
        if robots.get(&robot.produces).unwrap_or(&0) >= max_costs.get(&robot.produces).unwrap() {
            continue;
        }

        if !skipped.insert(robot.produces) {
            continue;
        }

        let mut robots_after = robots.clone();
        *robots_after.entry(robot.produces).or_insert(0) += 1;

        let mut resources_after = resources_after.clone();
        *resources_after.entry(Ore).or_insert(0) -= robot.cost_ore;
        *resources_after.entry(Clay).or_insert(0) -= robot.cost_clay;
        *resources_after.entry(Obsidian).or_insert(0) -= robot.cost_obsidian;

        max_geodes = run_iteration(
            time_remaining - 1,
            blueprint,
            &robots_after,
            &resources_after,
            // Reset all skipped after building a robot
            HashSet::new(),
            max_geodes,
            max_costs,
        )
        .max(max_geodes);
    }

    // Build nothing
    if consider_skipping {
        max_geodes = run_iteration(
            time_remaining - 1,
            blueprint,
            robots,
            &resources_after,
            skipped,
            max_geodes,
            max_costs,
        )
        .max(max_geodes);
    }

    max_geodes
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Robot {
    produces: Resource,
    cost_ore: i32,
    cost_clay: i32,
    cost_obsidian: i32,
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    robots: Vec<Robot>,
}

lazy_static! {
    static ref BP_RE: Regex = Regex::new(r"^Blueprint (?<id>\d+): Each ore robot costs (?<oo>\d+) ore. Each clay robot costs (?<co>\d+) ore. Each obsidian robot costs (?<bo>\d+) ore and (?<bc>\d+) clay. Each geode robot costs (?<go>\d+) ore and (?<gb>\d+) obsidian.$").unwrap();
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = BP_RE
            .captures(s)
            .ok_or(anyhow::anyhow!("Unable to match {}", s))?;

        let id = captures["id"].parse()?;
        let robots = vec![
            Robot {
                produces: Ore,
                cost_ore: captures["oo"].parse()?,
                cost_clay: 0,
                cost_obsidian: 0,
            },
            Robot {
                produces: Clay,
                cost_ore: captures["co"].parse()?,
                cost_clay: 0,
                cost_obsidian: 0,
            },
            Robot {
                produces: Obsidian,
                cost_ore: captures["bo"].parse()?,
                cost_clay: captures["bc"].parse()?,
                cost_obsidian: 0,
            },
            Robot {
                produces: Geode,
                cost_ore: captures["go"].parse()?,
                cost_clay: 0,
                cost_obsidian: captures["gb"].parse()?,
            },
        ];

        // "High-tech" ones in front, for better branch pruning
        let robots = robots.into_iter().rev().collect();

        Ok(Blueprint { id, robots })
    }
}
