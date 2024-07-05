use anyhow::Result;

fn main() -> Result<()> {
    let mut row: usize = 1;
    let mut col: usize = 1;
    let mut value: u64 = 20151125;

    let target_row = 2947;
    let target_col = 3029;

    loop {
        if row == target_row && col == target_col {
            break;
        }

        value = (value * 252533) % 33554393;

        (row, col) = if row == 1 {
            (col + 1, 1)
        } else {
            (row - 1, col + 1)
        }
    }

    println!("{row}, {col}: {value}");

    Ok(())
}
