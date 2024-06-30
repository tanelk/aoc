use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let password = "vzbxkghb";

    let mut password = password.to_string();
    loop {
        password = increment(password);
        if is_valid(password.as_str()) {
            break;
        }
    }

    println!("{}", password);

    loop {
        password = increment(password);
        if is_valid(password.as_str()) {
            break;
        }
    }

    println!("{}", password);

    Ok(())
}

fn increment(s: String) -> String {
    let mut s = s.into_bytes();
    let mut i = s.len() - 1;
    loop {
        if s[i] == b'z' {
            s[i] = b'a';
            i -= 1;
        } else {
            s[i] += 1;
            break;
        }
    }
    String::from_utf8(s).unwrap()
}

fn is_valid(s: &str) -> bool {
    // Rule 2
    if s.bytes().any(|c| c == b'i' || c == b'o' || c == b'l') {
        return false;
    }

    // Rule 1
    if !s
        .bytes()
        .tuple_windows()
        .any(|(a, b, c)| a == b - 1 && a == c - 2)
    {
        return false;
    }

    // Rule 3
    let first = s.bytes().tuple_windows().find_position(|(a, b)| a == b);
    if let Some((i, _)) = first {
        s.bytes().tuple_windows().skip(i + 2).any(|(a, b)| a == b)
    } else {
        false
    }
}
