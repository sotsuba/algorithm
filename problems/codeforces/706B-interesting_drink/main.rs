fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution_1::solve(tc);
}

mod solution_1 {
    use super::*;
    pub fn solve(tc: TestCase) {
        let mut nums = tc.nums;

        nums.sort_unstable();
        let max = *nums.last().unwrap();
        let mut cnt = vec![0_usize; max + 1];
        for &num in &nums {
            cnt[num] += 1;
        }

        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
        }

        for &coin in &tc.queries {
            let coin = coin.min(max);
            println!("{}", cnt[coin]);
        }
    }
}


mod solution_2 {
    use super::*;
    pub fn solve(tc: TestCase) {
        let mut nums = tc.nums;

        nums.sort_unstable();
        
        for coin in &tc.queries {
            println!("{}", nums.partition_point(|&x| x <= *coin));
        }
    }
}

pub struct TestCase {
    nums: Vec<usize>,
    queries: Vec<usize>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let nums = (0..n).map(|_| sc.next()).collect();
        let q = sc.next();
        let queries = (0..q).map(|_| sc.next()).collect();
        Self { nums, queries }
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

pub struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Scanner { iter: input.split_whitespace() }
    }
    fn next<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}