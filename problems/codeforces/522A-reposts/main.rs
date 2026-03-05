fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    
    let mut q = 1;
    // let mut q: usize = sc.next();
    while q > 0 {
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
        q -= 1;
    }
}

use std::collections::HashMap;

fn get_id(name: &str, mp: &mut HashMap<String, usize>) -> usize {
    if let Some(&id) = mp.get(name) {
        id
    }
    else {
        let id = mp.len();
        mp.insert(name.to_string(), id);
        id
    }
}


mod solution {
    use super::*;
    fn dfs(u: usize, p: usize, adj: & [Vec<usize>], depth: &mut [usize]) {
        for &v in &adj[u] {
            if v == p { continue; }
            depth[v] = depth[u] + 1;
            dfs(v, u, adj, depth);
        }
    }
    pub fn solve(tc: TestCase) {
        let TestCase { n, root, adj } = tc;
        let mut depth = vec![1_usize; n + 1];
        dfs(root, n + 1, &adj, &mut depth);
        println!("{:?}", *depth.iter().max().unwrap());
    }
}

pub struct TestCase {
    n: usize,
    root: usize, 
    adj: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next::<usize>() + 1;


        let mut adj = vec![vec![]; n + 1];
        let mut mp: HashMap<String, usize> = HashMap::new();
        for _ in 2..=n {
            let u_str = sc.next::<String>().to_lowercase();
            let _: String = sc.next();
            let v_str = sc.next::<String>().to_lowercase();
            let u = get_id(&u_str, &mut mp);
            let v = get_id(&v_str, &mut mp);

            adj[u].push(v);
            adj[v].push(u);
        }
        let root = get_id("polycarp", &mut mp);
        Self { n, root, adj }
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