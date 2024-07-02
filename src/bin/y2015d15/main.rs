use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d15/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut ingredients = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let ingredient: Ingredient = line.parse()?;

        println!("{:?}", ingredient);
        ingredients.push(ingredient);
    }

    // This is very inefficient way to get all the combinations.
    let max = (0..ingredients.len())
        .map(|_| 1..=100)
        .multi_cartesian_product()
        .filter(|amounts| amounts.iter().sum::<i32>() == 100)
        .filter_map(|amounts| {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;

            for (amount, ingredient) in amounts.iter().zip(&ingredients) {
                capacity += amount * ingredient.capacity;
                durability += amount * ingredient.durability;
                flavor += amount * ingredient.flavor;
                texture += amount * ingredient.texture;
                calories += amount * ingredient.calories;
            }

            capacity = capacity.max(0);
            durability = durability.max(0);
            flavor = flavor.max(0);
            texture = texture.max(0);
            calories = calories.max(0);

            if calories == 500 {
                Some(capacity * durability * flavor * texture)
            } else {
                None
            }
        })
        .max();

    println!("{:?}", max);

    Ok(())
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
        }

        let cap = RE
            .captures(s)
            .ok_or(anyhow::anyhow!("Unable to match {}", s))?;

        let capacity = cap.get(2).unwrap().as_str().parse()?;
        let durability = cap.get(3).unwrap().as_str().parse()?;
        let flavor = cap.get(4).unwrap().as_str().parse()?;
        let texture = cap.get(5).unwrap().as_str().parse()?;
        let calories = cap.get(6).unwrap().as_str().parse()?;

        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}
