fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;   
    fn dfs(u: usize, adj: &[Vec<usize>], dp: &mut [usize]) {
        dp[u] = 0;
        for &v in &adj[u] {
            dfs(v, adj, dp);
            dp[u] += dp[v] + 1;
        }
    }
    
    pub fn solve(tc: TestCase) {
        let TestCase { n, adj } = tc;
        let mut dp = vec![0_usize; n + 1];

        dfs(1, &adj, &mut dp);
        for i in 1..=n {
            print!("{} ", dp[i]);
        }
        println!();
    }
}

pub struct TestCase {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let mut adj = vec![vec![]; n + 1];
        for v in 2..=n {
            let u: usize = sc.next();
            adj[u].push(v);
        }
        Self { n, adj }
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