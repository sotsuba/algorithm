fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let t: usize = 1;
    for _ in 0..t { 
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
    }
}

pub struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let parent: Vec<_>= (0..n).map(|i| i).collect();
        let size = vec![1; n];
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb { return; }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }


    fn comp_size(&mut self, x: usize) -> i64 {
        let r = self.find(x);
        self.size[r]
    }
}

mod solution {
    use super::*;

    pub fn solve(tc: TestCase) {
        let TestCase { n, edges } = tc;
        
        let mut dsu = [DSU::new(n), DSU::new(n)];

        for &(u, v, c) in &edges {
            dsu[c as usize].union(u, v);
        }

        let mut ans = 0;
        for v in 0..n {
            let a = dsu[0].comp_size(v) - 1;
            let b = dsu[1].comp_size(v) - 1;
            ans += a + b + a * b;
        }
        println!("{}", ans);
    }
}



pub struct TestCase {
    n: usize,
    edges: Vec<(usize, usize, i32)>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let edges: Vec<_> = (0..n-1).map(|_| {
            let u = sc.next::<usize>() - 1;
            let v = sc.next::<usize>() - 1;
            let c = sc.next::<i32>();
            (u, v, c)
        }).collect();

        Self { n, edges }
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