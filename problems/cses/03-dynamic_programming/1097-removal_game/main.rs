pub struct TestCase {
    n: usize, 
    nums: Vec<i64>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let nums = (0..n).map( |_| sc.next() ).collect();
        Self { n, nums }
    }
}

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(&tc);
}   


mod solution {
    use super::*;
    pub fn solve(tc: &TestCase) {
        let n = tc.n;
        let nums = &tc.nums;
        let total: i64 = nums.iter().sum();

        let mut dp = vec![vec![0_i64; n]; n];
        for size in 0..=n {
            for i in 0..(n - size) {
                let j = i + size;
                let scenario_1 = nums[i] - if i + 1 < n { dp[i + 1][j] } else { 0 };
                let scenario_2 = nums[j] - if j as i32 - 1 >= 0 { dp[i][j - 1] } else { 0 };
                dp[i][j] = std::cmp::max(scenario_1, scenario_2);
            }
        }
        println!("{}", (total + dp[0][n - 1]) / 2);
    }
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

pub struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

