fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let n: usize = sc.next();
    let nums: Vec<usize> = (0..n).map(|_| { sc.next() }).collect();
    
    let mut ans = 1;
    let mut dp = vec![1_usize; n];
    for i in 1..n {
        dp[i] += if nums[i] >= nums[i - 1] { dp[i - 1] } else { 0 };
        ans = ans.max(dp[i]);
    }

    println!("{}", ans);
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

