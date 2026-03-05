fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs1(u: usize, p: usize, adj: &[Vec<usize>], sum: &mut [usize], dp: &mut [usize]) {
        for &v in &adj[u] {
            if v == p { continue; }
            dfs1(v, u, adj, sum, dp);
            sum[u] += sum[v];
            dp[u] += sum[v] + dp[v];
        }
    }

    fn dfs2(u: usize, p: usize, adj: &[Vec<usize>], total: usize, sum: &[usize], dp: &mut [usize]) {
        for &v in &adj[u] {
            if v == p { continue; }
            dp[v] = dp[u] + total - 2 * sum[v];
            dfs2(v, u, adj, total, sum, dp);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, score, adj } = tc;
        
        let mut sum = score.clone();
        let mut dp = vec![0_usize; n + 1];
        let root = 1;
        dfs1(root, 0, &adj, &mut sum, &mut dp);
        
        let total: usize = score.iter().sum();
        dfs2(root, 0, &adj, total, &sum, &mut dp);
        
        println!("{}", *dp.iter().max().unwrap());
    }
}

pub struct TestCase {
    n: usize,
    score: Vec<usize>,
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();

        let mut score = vec![0_usize; n + 1];
        for i in 1..=n { score[i] = sc.next(); }
        
        let mut adj = vec![vec![]; n + 1];
        for _ in 2..=n {
            let u: usize = sc.next();
            let v: usize = sc.next();

            adj[u].push(v);
            adj[v].push(u);
        }

        Self { n, score, adj }
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