use std::io::{self, BufRead};
use std::iter::{IntoIterator};

fn get_numbers() -> Vec<i32> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let numbers: Vec<i32> = handle.lines()
        .map_while(Result::ok) 
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>())
                .map_while(Result::ok) 
                .collect::<Vec<_>>()
        })
        .collect();
    
    numbers
}

fn main() {
    let mut numbers = get_numbers().into_iter();   
    let _ = numbers.next().unwrap();


    let mut prev: i32 = match numbers.next() {
        Some(val) => val,
        None => return,
    }; 

    let mut total_moves: i64 = 0;

    for cur in numbers {
        if cur < prev {
            total_moves += (prev - cur) as i64;
        }
        else {
            prev = cur;
        }
    }
    println!("{total_moves}");
}

# https://cses.fi/problemset/task/1094/
