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
    
    fn dp(mask: usize, prev: usize, n: usize, dishes: &[i64], combs: &[Vec<i64>], cache: &mut Vec<Vec<i64>>) -> i64 {
        if mask == 0 {  return 0; }
        
        if cache[mask][prev] != -1 { return cache[mask][prev] }

        let mut ans = 0;
        
        for next in 0..n {
            if (mask >> next) & 1 == 0 { continue; }
            let next_mask = mask ^ 1 << next;

            ans = ans.max(dp(next_mask, next, n, dishes, combs, cache) + dishes[next] + combs[prev][next]);
        }
        cache[mask][prev] = ans;
        ans
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, m, dishes, combs } = tc;
        let mut cache = vec![vec![-1_i64; n + 1]; 1 << n];

        let mut ans = 0;
        for mask in 0..(1 << n) {
            if (mask as i32).count_ones() as usize == m {
                ans = ans.max(dp(mask, n, n, &dishes, &combs, &mut cache));
            }
        } 
        println!("{}", ans);
    }
}

pub struct TestCase {
    n: usize,
    m: usize,
    dishes: Vec<i64>,
    combs: Vec<Vec<i64>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let m = sc.next();
        let k = sc.next();
        let dishes = (0..n).map(|_| sc.next()).collect();
        
        let mut combs = vec![vec![0_i64; n + 1]; n + 1];
        for _ in 0..k {
            let u = sc.next::<usize>() - 1;
            let v = sc.next::<usize>() - 1;
            let w = sc.next::<i64>();
            combs[u][v] = w;
        }

        Self { n, m, dishes, combs }        
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