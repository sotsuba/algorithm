pub struct Input {
    n: usize, 
    nums: Vec<usize>,
}

impl Input {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let nums = (0..n).map( |_| sc.next() ).collect();
        Self { n, nums }
    }
}

pub struct BIT {
    n: usize,
    inner: Vec<usize>,
}

impl BIT {
    fn new(n: usize) -> Self { Self { n, inner: vec![0_usize; n +1] } }
    
    fn update(&mut self, idx: usize, val: usize) {
        let mut idx = idx;

        while idx <= self.n {
            self.inner[idx] = self.inner[idx].max(val);
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, idx: usize) -> usize {
        let mut ans = 0;
        let mut idx = idx;

        while idx > 0 {
            ans = ans.max(self.inner[idx as usize]);
            idx &= idx - 1;
        }

        ans 
    }
}

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let input = Input::new(&mut sc);
    println!("{}", solution::solve(&input));
}   

#[allow(dead_code)]
mod solution {
    use super::*;

    pub fn solve(input: &Input) -> usize {
        let n = input.n;
        let nums = &input.nums;
        let mut unique_vals = nums.clone();
        unique_vals.sort();
        unique_vals.dedup();

        let mut bit = BIT::new(n);
        let mut ans = 1;

        for i in 0..n {
            let pos = unique_vals.binary_search(&nums[i]).unwrap() + 1;
            let val = bit.query(pos - 1) + 1;
            bit.update(pos, val);
            ans = ans.max(val);
        }

        ans 
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

