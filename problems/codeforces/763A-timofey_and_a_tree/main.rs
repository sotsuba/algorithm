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
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], colors: &[usize], tar1: &mut usize, tar2: &mut usize) {
        for &v in &adj[u] {
            if v == p { continue; }
            if colors[v] != colors[u] {
                *tar1 = u;
                *tar2 = v;
                return;
            }
            dfs(v, u, adj, colors, tar1, tar2);
        }
    }

    fn dfs_unique(u: usize, p: usize, adj: &[Vec<usize>], colors: &[usize], ans: &mut bool) {
        if *ans == false { return; }

        for &v in &adj[u] {
            if v == p { continue; }
            if p != 0 && colors[v] != colors[u] {
                *ans = false;
                return;
            }
            dfs_unique(v, u, adj, colors, ans);
        }
    }

    pub fn solve(tc: TestCase) {
        let TestCase { adj, colors } = tc;
        let mut tar1 = 1;
        let mut tar2 = 1;

        dfs(1, 0, &adj, &colors, &mut tar1, &mut tar2);

        let mut ans = true;
        dfs_unique(tar1, 0, &adj, &colors, &mut ans);
        if ans {
            println!("YES\n{}", tar1);
        }
        else {
            ans = true;
            dfs_unique(tar2, 0, &adj, &colors, &mut ans);
            if ans {
                println!("YES\n{}", tar2);
            }
            else {
                println!("NO");
            }
        }


    }
}

pub struct TestCase {
    adj: Vec<Vec<usize>>,
    colors: Vec<usize>,
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

        let mut colors = vec![0_usize; n + 1];
        for i in 1..=n { colors[i] = sc.next(); }

        Self { adj, colors }
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