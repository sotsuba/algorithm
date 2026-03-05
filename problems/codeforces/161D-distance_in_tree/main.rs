fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    
    fn dfs(u: usize, p: usize, k: usize, adj: &[Vec<usize>], dp: &mut [Vec<usize>], ans: &mut usize) {
        dp[u][0] = 1;


        for &v in &adj[u] {
            if v == p { continue; }
            dfs(v, u, k, adj, dp, ans);

            for d in 0..k {
                *ans += dp[u][k - d - 1] * dp[v][d];
            }
            
            for size in 1..=k {
                dp[u][size] += dp[v][size - 1];
            }
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, k, adj } = tc;
        let mut dp = vec![vec![0_usize; k + 1]; n + 1];
        let root = 1;
        let mut ans = 0;
        dfs(root, 0, k, &adj, &mut dp, &mut ans);
        
        println!("{}", ans);
    }
    
}

pub struct TestCase {
    n: usize,
    k: usize,
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let k = sc.next();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for _ in 1..n {
            let u: usize = sc.next();
            let v: usize = sc.next();
            adj[u].push(v);
            adj[v].push(u);
        }

        Self { n, k, adj }
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