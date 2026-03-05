fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], depth: &mut [i32], subtree_size: &mut [i32]) {
        subtree_size[u] = 1;

        for &v in &adj[u] {
            if v == p { continue; }
            depth[v] = depth[u] + 1;
            dfs(v, u, adj, depth, subtree_size);
            subtree_size[u] += subtree_size[v];
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, k, adj } = tc;
        
        let mut depth = vec![0_i32; n + 1];
        let mut subtree_size = vec![0_i32; n + 1];
        
        dfs(1, 0, &adj, &mut depth, &mut subtree_size);

        let mut score: Vec<i64> = (1..=n).map(|i| (depth[i] - (subtree_size[i] - 1)) as i64 ).collect();
        score.sort_by(|a, b| b.cmp(a));

        let ans: i64 = score[..k].iter().sum::<i64>();

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
        let mut adj = vec![vec![]; n + 1];
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