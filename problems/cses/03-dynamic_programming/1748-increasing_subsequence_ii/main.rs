const MOD: usize = 1_000_000_007;

pub struct BIT {
    n: usize,
    inner: Vec<usize>,
}

impl BIT {
    pub fn new(n: usize) -> Self { Self { n, inner: vec![0_usize; n + 1] } }

    pub fn get_prefix(&self, index: i32) -> usize {
        let mut index = index;
        
        let mut ans = 0;
        while index > 0 {
            ans += self.inner[index as usize];
            index -= index & (-index);
        }

        ans
    }

    pub fn add(&mut self, index: i32, value: usize) {
        let mut index = index;
        let n = self.n as i32;

        while index <= n {
            self.inner[index as usize] = (self.inner[index as usize] + value) % MOD;
            index += index & (-index);
        }
    }
}


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

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let input = Input::new(&mut sc);
    println!("{}", fast_solution::solve(&input));
}   


mod fast_solution {
    use super::*;

    pub fn solve(input: &Input) -> usize {
        let n = input.n;
        let nums = &input.nums;
        let mut vals = nums.clone();
        vals.sort();
        vals.dedup();
        
        let mut bit = BIT::new(n);

        let mut ans = 0;
        for i in 0..n {
            let pos: i32 = vals.binary_search(&nums[i]).unwrap() as i32 + 1;
            let prefix_sum = bit.get_prefix(pos - 1) + 1;
            ans = (ans + prefix_sum) % MOD;

            bit.add(pos, prefix_sum);
            
        }
        ans 
    }
}

#[allow(dead_code)]
mod slow_solution {
    use super::*;

    pub fn solve(input: &Input) -> usize {
        let n = input.n;
        let nums = &input.nums;

        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                dp[i] += if nums[i] > nums[j] { dp[j] } else { 0 };
            }
        }

        let mut ans = 0;
        for x in dp { ans = (ans + x) % MOD; }

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

