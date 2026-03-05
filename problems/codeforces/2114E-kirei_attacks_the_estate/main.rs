fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let mut q = 1;
    let mut q: usize = sc.next();
    while q > 0 {
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
        q -= 1;
    }
}

mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], score: &[i64], min_slice: &mut [i64], max_slice: &mut [i64]) {
        for &v in &adj[u] {
            if p == v { continue; }
            max_slice[v] = score[v] - min_slice[u].min(0);
            min_slice[v] = score[v] - max_slice[u].max(0);
            dfs(v, u, adj, score, min_slice, max_slice);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, score, adj } = tc;
        let mut min_slice = vec![0_i64; n + 1];
        let mut max_slice = vec![0_i64; n + 1];
        max_slice[1] = score[1];
        min_slice[1] = score[1];

        dfs(1, 0, &adj, &score, &mut min_slice, &mut max_slice);
        for i in 1..=n {
            print!("{} ", max_slice[i]);
        }
        println!();
    }
}

pub struct TestCase {
    n: usize,
    score: Vec<i64>,
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let mut score = vec![0_i64; n + 1];
        for i in 1..=n {
            score[i] = sc.next();
        }

        let mut adj = vec![vec![]; n + 1];
        for _ in 2..=n {
            let u: usize = sc.next();
            let v: usize = sc.next();
            adj[v].push(u);
            adj[u].push(v);
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