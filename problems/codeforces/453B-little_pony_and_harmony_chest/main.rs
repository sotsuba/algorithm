fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let t: usize = 1;
    for _ in 0..t { 
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
    }
}

mod solution {
    use super::*;

    const PRIME: [usize; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];


    fn prime_factors(x: usize) -> usize {
        let mut x = x;
        let mut ans = 0_usize;

        for (i, &p) in PRIME.iter().enumerate() {
            if x % p == 0 {
                ans |= 1 << i;
                while x % p == 0 {
                    x /= p;
                }
            } 
        }
        ans
    }

    fn dp(mask: usize, i: usize, n: usize, nums: &[usize], prime: &[usize], cache: &mut [Vec<(usize, usize)>]) -> usize {
        if i == n {
            return 0;
        }

        if cache[mask][i].0 != usize::MAX { return cache[mask][i].0; }

        let mut best_val  = 1;
        let mut best_cost = dp(mask, i + 1, n, nums, prime, cache) + nums[i].abs_diff(1);
        
        for cand_val in 2..=59 {
            if mask & prime[cand_val] == prime[cand_val] {
                let new_mask = mask ^ prime[cand_val];
                let cand_cost = dp(new_mask, i + 1, n, nums, prime, cache) + nums[i].abs_diff(cand_val);

                if cand_cost < best_cost {
                    best_val  = cand_val;
                    best_cost = cand_cost;
                }
            }
        }   
        
        cache[mask][i] = (best_cost, best_val);
        best_cost
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, nums } = tc;
        let prime = (0..=60).map(|i| if i <= 1 { 0 } else { prime_factors(i)}).collect::<Vec<_>>();        

        let total_mask = (1 << 16) - 1;
        let mut cache = vec![vec![(usize::MAX, 1); n + 1]; total_mask + 1];
        
        dp(total_mask, 0, n, &nums, &prime, &mut cache);

        let mut mask = total_mask;
        for i in 0..n {
            let val = cache[mask][i].1;

            mask ^= prime[val];
            print!("{} ", val);
        }
        println!();

    }
}

pub struct TestCase {
    n: usize,
    nums: Vec<usize>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let nums = (0..n).map(|_| sc.next()).collect();
        Self { n, nums }
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