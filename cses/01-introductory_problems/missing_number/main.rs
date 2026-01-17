use std::io::{self, Read};

fn main() {
    let handle = io::stdin().lock();
    let mut line = String::new();

    // Read N 
    handle.read_line(&mut line).ok();
    let n: u64 = line.trim().parse().unwrap_or(0);

    let mut res: u64 = 0;
    for i in 1..=n {
        res ^= i;
    }

    // Read array
    handle.read_line(&mut line).unwrap();
    for token in line.split_whitespace() {
        if let Ok(val) = token.parse::<u64>() {
            res ^= val;
        }
    }
    println!("{res}");
}

// https://cses.fi/problemset/task/1083
