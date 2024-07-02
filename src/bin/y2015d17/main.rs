use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let containers = vec![
        43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38,
    ];

    let c = containers
        .into_iter()
        .powerset()
        .filter(|subset| subset.iter().sum::<i32>() == 150)
        .counts_by(|subset| subset.len());

    println!("Matching combinations: {:?}", c);

    Ok(())
}
