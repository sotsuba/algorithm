fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let n: usize = sc.next();

    let mut types: Vec<usize> = (0..3).map(|_| { sc.next() }).collect();
    types.sort_unstable();

    let mut dp = vec![0; n + 1];
    for &t in &types { if t <= n { dp[t] = 1; } }

    for len in 1..=n {
        for &t in &types {
            if t > len || dp[len - t] == 0 { continue; }
            dp[len] = dp[len].max(dp[len - t] + 1);
        }
    }
    println!("{}", dp[n]);
}   








































use std::io::{self, Read};
use std::str::FromStr;

fn adapted_input() -> String {
    match cfg!(debug_assertions) {
        true => std::fs::read_to_string("input.txt").unwrap(),
        false => { 
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    }
}

struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

