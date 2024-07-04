use anyhow::Result;

fn main() -> Result<()> {
    let goal_count = 33_100_000;

    let first_house = (1..).find(|&i| {
        let mut present_count = 0;
        let sqrt = (i as f64).sqrt() as i32;

        for j in 1..=sqrt {
            if i % j != 0 {
                continue;
            }

            // This is both the number of another elf who visits this house, but also how many houses the i-th elf has visited.
            let other = i / j;

            if other <= 50 {
                present_count += 11 * j;
            }

            if other != j && j <= 50 {
                present_count += 11 * other;
            }
        }

        present_count >= goal_count
    });

    println!("{:?}", first_house);

    Ok(())
}
