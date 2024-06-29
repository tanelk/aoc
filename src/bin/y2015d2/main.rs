use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?.join("src/bin/y2015d2/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in reader.lines() {
        let line = line?;
        let parsed = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        match parsed.as_slice() {
            [l, w, h] => {
                let lw = l * w;
                let wh = w * h;
                let hl = h * l;
                let extra = lw.min(wh).min(hl);
                let area = 2 * (lw + wh + hl) + extra;
                total_area += area;

                let ribbon = 2 * (l + w + h - l.max(w).max(h));
                let bow = l * w * h;

                total_ribbon += ribbon + bow;

            },
            _ => println!("Invalid line: {}", line)
        }
    }

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);

    Ok(())
}
