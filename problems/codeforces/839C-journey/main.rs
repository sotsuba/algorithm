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
    fn dfs(prob: f64, depth: f64, u: usize, p: usize, adj: & [Vec<usize>], ans: &mut f64) {
        let child_count = if p == 0 { adj[u].len() } else { adj[u].len() - 1 };

        if child_count == 0 {
            *ans += depth * prob;
            return;
        } 

        let next_prob = prob / child_count as f64;
        for &v in &adj[u] {
            if v == p { continue; }
            dfs(next_prob, depth + 1.0, v, u, adj, ans);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { adj } = tc;
        let mut ans = 0.0;

        let root = 1;
        dfs(1.0, 0.0, root, 0, &adj, &mut ans);
        println!("{:.15}", ans);
    }
}

pub struct TestCase {
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next::<usize>();
        let mut adj = vec![vec![]; n + 1];
        for _ in 1..n {
            let u = sc.next::<usize>();
            let v = sc.next::<usize>();
            
            adj[u].push(v);
            adj[v].push(u);
        }

        Self { adj }
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