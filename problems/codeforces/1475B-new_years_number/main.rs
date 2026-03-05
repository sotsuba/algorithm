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
        let num_2021 = tc.n % 2020;
        
        let sum_2020 = tc.n - num_2021 * 2021;

        if sum_2020 < 0 || sum_2020 % 2020 != 0  { 
            println!("NO");
        }
        else {
            println!("YES");
        }
    }
}

pub struct TestCase {
    n: i64,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        Self { n }
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