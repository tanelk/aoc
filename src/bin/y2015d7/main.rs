use crate::Operation::Literal;
use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d7/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut operations = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let assignment: Assignment = line.parse()?;
        operations.insert(assignment.target, assignment.operation);
    }

    let res = evaluate(
        operations
            .get("a")
            .ok_or(anyhow::anyhow!("Key 'a' not found"))?,
        &operations,
        &mut HashMap::new(),
    )?;
    println!("Result in 'a': {}", res);

    // Part2
    operations.insert("b".to_string(), Literal(res));
    let res = evaluate(
        operations
            .get("a")
            .ok_or(anyhow::anyhow!("Key 'a' not found"))?,
        &operations,
        &mut HashMap::new(),
    )?;
    println!("Result in 'a' after 2nd iter: {}", res);

    Ok(())
}

fn evaluate(
    op: &Operation,
    operations: &HashMap<String, Operation>,
    cache: &mut HashMap<String, u16>,
) -> Result<u16> {
    match op {
        Operation::Literal(lit) => Ok(*lit),
        Operation::Variable(var) => {
            if let Some(cached_value) = cache.get(var) {
                Ok(*cached_value)
            } else {
                let op = operations
                    .get(var)
                    .ok_or(anyhow::anyhow!("Key '{}' not found", var))?;
                let res = evaluate(op, operations, cache)?;
                cache.insert(var.to_string(), res);
                Ok(res)
            }
        }
        Operation::Not(op) => Ok(!evaluate(op, operations, cache)?),
        Operation::And(l, r) => {
            let l = evaluate(l, operations, cache)?;
            let r = evaluate(r, operations, cache)?;
            Ok(l & r)
        }
        Operation::Or(l, r) => {
            let l = evaluate(l, operations, cache)?;
            let r = evaluate(r, operations, cache)?;
            Ok(l | r)
        }
        Operation::LeftShift(l, r) => {
            let l = evaluate(l, operations, cache)?;
            let r = evaluate(r, operations, cache)?;
            Ok(l << r)
        }
        Operation::RightShift(l, r) => {
            let l = evaluate(l, operations, cache)?;
            let r = evaluate(r, operations, cache)?;
            Ok(l >> r)
        }
    }
}

#[derive(Debug)]
enum Operation {
    Literal(u16),
    Variable(String),
    Not(Box<Operation>),
    And(Box<Operation>, Box<Operation>),
    Or(Box<Operation>, Box<Operation>),
    LeftShift(Box<Operation>, Box<Operation>),
    RightShift(Box<Operation>, Box<Operation>),
}

#[derive(Debug)]
struct Assignment {
    operation: Operation,
    target: String,
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(" -> ").collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!(format!("Invalid assignment: {}", s)));
        }

        let target = parts[1].to_string();
        let operation = parts[0].parse()?;

        Ok(Assignment { operation, target })
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.len() {
            1 => {
                if let Ok(literal) = parts[0].parse() {
                    Ok(Operation::Literal(literal))
                } else {
                    Ok(Operation::Variable(parts[0].to_string()))
                }
            }
            2 => {
                if parts[0] == "NOT" {
                    let operation = parts[1].parse()?;
                    Ok(Operation::Not(Box::new(operation)))
                } else {
                    Err(anyhow::anyhow!(format!("Invalid operation: {}", s)))
                }
            }
            3 => {
                let operation1 = parts[0].parse()?;
                let operation2 = parts[2].parse()?;

                match parts[1] {
                    "AND" => Ok(Operation::And(Box::new(operation1), Box::new(operation2))),
                    "OR" => Ok(Operation::Or(Box::new(operation1), Box::new(operation2))),
                    "LSHIFT" => Ok(Operation::LeftShift(
                        Box::new(operation1),
                        Box::new(operation2),
                    )),
                    "RSHIFT" => Ok(Operation::RightShift(
                        Box::new(operation1),
                        Box::new(operation2),
                    )),
                    _ => Err(anyhow::anyhow!(format!("Invalid operation: {}", s))),
                }
            }
            _ => Err(anyhow::anyhow!(format!("Invalid operation: {}", s))),
        }
    }
}
