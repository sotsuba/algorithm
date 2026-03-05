fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let mut q: usize = sc.next();
    while q > 0 {
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);

        q -= 1;
    }
}

mod solution {
    use super::*;
    pub fn solve(tc: TestCase) {
        let cnt1 = tc.nums.iter().filter(|&&x| x == 1).count();
        let cnt2 = tc.n - cnt1;

        if cnt1 % 2 == 1 || (cnt2 % 2 == 1 && cnt1 < 2)  { 
            println!("NO");
        }
        else {
            println!("YES");
        }
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