fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn manage_best(best1: &mut usize, best2: &mut usize, cand: usize) {
        if cand > *best1 {
            *best2 = *best1;
            *best1 = cand;
        }
        else if cand > *best2 {
            *best2 = cand;
        }
    } 
    
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], downward_side: &mut [usize], diameter: &mut usize) {
        let mut best1 = 0_usize;
        let mut best2 = 0_usize;
        
        for &v in &adj[u] { 
            if v == p { continue; }
            dfs(v, u, adj, downward_side, diameter);
            
            let cand = downward_side[v] + 1;
            manage_best(&mut best1, &mut best2, cand);
        }
        downward_side[u] = best1;
        *diameter = (*diameter).max(best1 + best2);
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, adj } = tc;
        let mut downward_side = vec![0_usize; n + 1];
        let mut diameter = 0;
        
        dfs(1, 0, &adj, &mut downward_side, &mut diameter);
        println!("{}", diameter);
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
        for _ in 1..n {
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