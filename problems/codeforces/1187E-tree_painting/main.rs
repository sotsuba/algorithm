fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], sub: &mut [usize], dp: &mut [usize]) {
        for &v in &adj[u] {
            if v == p { continue; }
            dfs(v, u, adj, sub, dp);
            sub[u] += sub[v];
            dp[u] += dp[v];
        }
        dp[u] += sub[u];
    }

    fn dfs_final (u: usize, p: usize, adj: &[Vec<usize>], sub: &mut [usize], dp: &mut [usize], n: usize) {
        for &v in &adj[u] {
            if v == p { continue; }
            dp[v] = dp[u] + n - 2 * sub[v];  
            dfs_final(v, u, adj, sub, dp, n);
        }
    } 


    pub fn solve(tc: TestCase) {
        let TestCase { n, adj } = tc;
        let mut sub = vec![1_usize; n + 1];
        let mut dp = vec![0_usize; n + 1];
        let root = 1;
        
        dfs(root, 0, &adj, &mut sub, &mut dp);
        dfs_final(root, 0, &adj, &mut sub, &mut dp, n);

        println!("{}", *dp.iter().max().unwrap());
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
        for _ in 2..=n {
            let u: usize = sc.next();
            let v: usize = sc.next();
            adj[u].push(v);
            adj[v].push(u);
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