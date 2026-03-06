fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    fn dfs1(u: usize, p: usize, adj: &[Vec<usize>], colors: &[i32], down: &mut [i32]) {
        for &v in &adj[u] {
            if v == p { continue; }
            dfs1(v, u, adj, colors, down);
            down[u] += down[v].max(0);
        }
        down[u] = down[u];
    }

    fn dfs2(u: usize, p: usize, adj: &[Vec<usize>], colors: &[i32], down: &mut [i32], up: &mut[i32], ans: &mut [i32]) {
        ans[u] = down[u] + up[u];

        for &v in &adj[u] {
            if v == p { continue; }
            up[v] = (ans[u] - down[v].max(0)).max(0);
            dfs2(v, u, adj, colors, down, up, ans);
        }
    }


    pub fn solve(tc: TestCase) {
        let TestCase { n, colors, adj } = tc;
        let mut down = colors.clone();
        let mut up = vec![0_i32; n + 1];
        
        let root = 1;
        dfs1(root, 0, &adj, &colors, &mut down);
        let mut ans = vec![0_i32; n + 1];
        ans[root] = down[root];

        dfs2(root, 0, &adj, &colors, &mut down, &mut up, &mut ans);
        for i in 1..=n {
            print!("{} ", ans[i]);
        }
        println!();
    }
}

pub struct TestCase {
    n: usize,
    colors: Vec<i32>,
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let mut colors = vec![0_i32; n + 1];
        let mut adj = vec![vec![]; n + 1];

        for i in 1..=n { colors[i] = if sc.next::<i32>() == 1 { 1 } else { -1 }; }

        for _ in 2..=n {
            let u: usize = sc.next();
            let v: usize = sc.next();

            adj[u].push(v);
            adj[v].push(u);
        }

        Self { n, colors, adj }
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