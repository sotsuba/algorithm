fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs1(u: usize, p: usize, adj: &[Vec<(usize, i32)>], ans: &mut [i32]) {
        for &(v, cost) in &adj[u] {
            if v == p { continue; }
            dfs1(v, u, adj, ans);
            ans[u] += ans[v] + cost;
        }

    }
    fn dfs2(u: usize, p: usize, adj: &[Vec<(usize, i32)>], ans: &mut [i32]) {
        for &(v, cost) in &adj[u] {
            if v == p { continue; }
            ans[v] = ans[u] + if cost == 1 { -1 } else { 1 };
            dfs2(v, u, adj, ans);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, adj } = tc;
        let mut ans = vec![0_i32; n + 1];
        let root = 1;

        dfs1(root, 0, &adj, &mut ans);
        dfs2(root, 0, &adj, &mut ans);

        let best = (1..=n).map(|i| ans[i]).min().unwrap();
        println!("{}", best);
        for i in 1..=n {
            if ans[i] == best {
                print!("{} ", i);
            }
        }
        println!();
    }
}

pub struct TestCase {
    n: usize,
    adj: Vec<Vec<(usize, i32)>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let mut adj = vec![vec![]; n + 1];
        for _ in 2..=n {
            let u: usize = sc.next();
            let v: usize = sc.next();
            adj[u].push((v, 0));
            adj[v].push((u, 1));
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