fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution_1::solve(tc);
}

mod solution_1 {
    use super::*;
    pub fn solve(tc: TestCase) {
        let TestCase { n, k, nums } = tc; 

        let mut slice_sum: usize = nums[..k].iter().sum();
        let mut min_slice = slice_sum;
        let mut index = 1;

        for i in k..n {
            slice_sum = slice_sum + nums[i] - nums[i - k];
            if min_slice > slice_sum {
                min_slice = slice_sum;
                index = i - k + 2;
            }
        }        
        println!("{}", index);
    }   
}

pub struct TestCase {
    n: usize,
    k: usize,
    nums: Vec<usize>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let k = sc.next();
        let nums = (0..n).map(|_| sc.next()).collect();
        Self { n, k, nums }
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