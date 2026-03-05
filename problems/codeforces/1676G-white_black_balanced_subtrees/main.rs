fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    
    let mut q: usize = sc.next();
    while q > 0 {
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
        q -= 1;
    }
}

mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], colors: &[i8], pen: &mut [i32]) {
        pen[u] = colors[u] as i32;
        for &v in &adj[u] {
            if v == p { continue; }
            dfs(v, u, adj, colors, pen);
            pen[u] += pen[v];
        }
    }


    pub fn solve(tc: TestCase) {
        let TestCase { n, adj, colors } = tc;
        let mut pen = vec![0_i32; n + 1];
        dfs(1, 0, &adj, &colors, &mut pen);
        let mut balanced_nodes = pen[1..].iter().filter(|&&x| x == 0).count();
        println!("{}", balanced_nodes);
    }
}

pub struct TestCase {
    n: usize,
    adj: Vec<Vec<usize>>,
    colors: Vec<i8>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let mut adj = vec![vec![]; n + 1];

        for v in 2..=n {
            let u: usize = sc.next();
            adj[u].push(v);
        }

        let mut colors = vec![0_i8; n + 1];
        let binding = sc.next::<String>();
        let s = binding.as_bytes();

        for i in 1..=n {
            colors[i] = if s[i - 1] == b'B' { 1 } else { -1 };
        }
        Self { n, adj, colors }
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