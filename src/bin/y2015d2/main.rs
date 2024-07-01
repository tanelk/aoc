use std::env;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d12/input.txt");
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    let v: Value = serde_json::from_str(&contents)?;

    println!("{}", sum_json(&v));

    Ok(())
}

fn sum_json(v: &Value) -> i64 {
    match v {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(sum_json).sum(),
        Value::Object(object) => {
            if object
                .values()
                .any(|v| v.as_str().is_some_and(|s| s == "red"))
            {
                0
            } else {
                object.values().map(sum_json).sum()
            }
        }
        _ => 0,
    }
}
