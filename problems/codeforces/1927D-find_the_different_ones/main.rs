fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let t: usize = sc.next();
    for _ in 0..t { 
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
    }
}

mod solution {
    use super::*;
    pub fn solve(tc: TestCase) {
        let TestCase { n, nums, queries } = tc;
        let mut next_diff = vec![0_usize; n];
        next_diff[n - 1] = 1_000_000_009;

        for i in (0..n - 1).rev() {
            next_diff[i] = if nums[i] == nums[i + 1] { next_diff[i + 1] + 1 } else { 1 };
        }

        for &(l, tmp) in &queries {
            let l = l - 1;
            let tmp = tmp - 1;
            let r = l + next_diff[l];
            if r <= tmp {
                println!("{} {}", l + 1, r + 1);
            }
            else {
                println!("-1 -1");
            }
        }
        println!();
        // println!("{:?}", &nums);
        // println!("{:?}", &next_diff);
    }
}

pub struct TestCase {
    n: usize,
    nums: Vec<usize>,
    queries: Vec<(usize, usize)>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let nums = (0..n).map(|_| sc.next()).collect();
        let q = sc.next();
        let queries = (0..q).map(|_| (sc.next(), sc.next())).collect();
        Self { n, nums, queries }
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