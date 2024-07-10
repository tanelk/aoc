use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;

fn main() -> Result<()> {
    let path = env::current_dir()?.join("src/bin/y2019d22/input.txt");
    let reader = BufReader::new(File::open(path)?);

    let mut shuffles = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("deal into new stack") {
            shuffles.push(Shuffle::DealIntoNewStack);
        } else if line.starts_with("cut") {
            let n = line.split_whitespace().last().unwrap().parse()?;
            shuffles.push(Shuffle::Cut(n));
        } else if line.starts_with("deal with increment") {
            let n = line.split_whitespace().last().unwrap().parse()?;
            shuffles.push(Shuffle::DealWithIncrement(n));
        } else {
            panic!("Unknown shuffle: {}", line);
        }
    }

    // Part 1
    let deck_size = 10007;
    let mut position = 2019;
    for shuffle in &shuffles {
        position = shuffle.track_position(deck_size, position);
    }
    println!("Part 1: {}", position);

    Ok(())
}

enum Shuffle {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl Shuffle {
    #[allow(dead_code)]
    fn invert(&self, deck_size: usize) -> Self {
        match self {
            Shuffle::DealIntoNewStack => Shuffle::DealIntoNewStack,
            Shuffle::Cut(n) => Shuffle::Cut(-n),
            Shuffle::DealWithIncrement(n) => Shuffle::DealWithIncrement(deck_size - *n),
        }
    }

    fn track_position(&self, deck_size: usize, position: usize) -> usize {
        debug_assert!(
            position < deck_size,
            "Position {} is out of bounds",
            position
        );

        match self {
            Shuffle::DealIntoNewStack => deck_size - position - 1,
            Shuffle::Cut(n) => {
                let n = *n;
                if n >= 0 {
                    (position + (deck_size - n as usize)) % deck_size
                } else {
                    (position + n.unsigned_abs()) % deck_size
                }
            }
            Shuffle::DealWithIncrement(n) => {
                let position = position as u128;
                let n = *n as u128;
                let deck_size = deck_size as u128;

                ((position * n) % deck_size) as usize
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Shuffle;

    #[test]
    fn test_deal_into_new_stack() {
        let deck_size = 10;
        let shuffle = Shuffle::DealIntoNewStack;
        let inverted = shuffle.invert(deck_size);

        assert_eq!(shuffle.track_position(deck_size, 0), 9);
        assert_eq!(shuffle.track_position(deck_size, 1), 8);

        for i in 0..deck_size {
            let new_position = shuffle.track_position(deck_size, i);
            assert_eq!(inverted.track_position(deck_size, new_position), i);
        }
    }

    #[test]
    fn test_cut_positive() {
        let deck_size = 10;
        let shuffle = Shuffle::Cut(3);
        let inverted = shuffle.invert(deck_size);

        assert_eq!(shuffle.track_position(deck_size, 0), 7);
        assert_eq!(shuffle.track_position(deck_size, 2), 9);
        assert_eq!(shuffle.track_position(deck_size, 3), 0);
        assert_eq!(shuffle.track_position(deck_size, 9), 6);

        for i in 0..deck_size {
            let new_position = shuffle.track_position(deck_size, i);
            assert_eq!(
                inverted.track_position(deck_size, new_position),
                i,
                "Failed for {}",
                i
            );
        }
    }

    #[test]
    fn test_cut_negative() {
        let deck_size = 10;
        let shuffle = Shuffle::Cut(-4);
        let inverted = shuffle.invert(deck_size);

        assert_eq!(shuffle.track_position(deck_size, 0), 4);
        assert_eq!(shuffle.track_position(deck_size, 2), 6);
        assert_eq!(shuffle.track_position(deck_size, 3), 7);
        assert_eq!(shuffle.track_position(deck_size, 9), 3);

        for i in 0..deck_size {
            let new_position = shuffle.track_position(deck_size, i);
            assert_eq!(
                inverted.track_position(deck_size, new_position),
                i,
                "Failed for {}",
                i
            );
        }
    }

    #[test]
    fn test_deal_with_increment() {
        let deck_size = 10;
        let shuffle = Shuffle::DealWithIncrement(3);
        let inverted = shuffle.invert(deck_size);

        assert_eq!(shuffle.track_position(deck_size, 0), 0);
        assert_eq!(shuffle.track_position(deck_size, 1), 3);
        assert_eq!(shuffle.track_position(deck_size, 2), 6);
        assert_eq!(shuffle.track_position(deck_size, 3), 9);
        assert_eq!(shuffle.track_position(deck_size, 4), 2);
        assert_eq!(shuffle.track_position(deck_size, 5), 5);

        for i in 0..deck_size {
            let new_position = shuffle.track_position(deck_size, i);
            assert_eq!(
                inverted.track_position(deck_size, new_position),
                i,
                "Failed for {}",
                i
            );
        }
    }
}
