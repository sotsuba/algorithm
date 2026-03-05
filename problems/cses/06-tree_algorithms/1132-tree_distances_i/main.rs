fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], down: &mut [usize]) {
        for &v in &adj[u] {
            if v == p { continue; }
            down[v] = down[u] + 1;
            dfs(v, u, adj, down);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase {n, adj} = tc;
        let mut down = vec![0_usize; n + 1];
        dfs(1, 0, &adj, &mut down);

        let endpoint_1 = down.iter().enumerate().max_by_key(|&(_i, &x)| x).map(|(i,_)| i).unwrap();
        let mut down1 = vec![0_usize; n + 1];
        dfs(endpoint_1, 0, &adj, &mut down1);
        
        let endpoint_2 = down1.iter().enumerate().max_by_key(|&(_i, &x)| x).map(|(i,_)| i).unwrap();
        let mut down2 = vec![0_usize; n + 1];
        dfs(endpoint_2, 0, &adj, &mut down2);
        
        for i in 1..=n {
            print!("{} ", down1[i].max(down2[i]));
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