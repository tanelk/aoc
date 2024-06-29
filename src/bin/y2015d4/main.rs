use md5;

fn main() -> std::io::Result<()> {
    let input = "ckczppom";

    for i in 0..i32::MAX {
        let concatenated =  format!("{}{}", input, i);
        let hash = md5::compute(concatenated);

        let enough_zeros = hash[0] == 0 && hash[1] == 0 && hash[2] < 16;
        if enough_zeros {
            println!("Found it! {}", i);
            break;
        }
    }


    Ok(())
}