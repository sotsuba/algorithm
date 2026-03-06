fn main() {
    solution::solve();
}

pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    n_comp: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let parent = (0..=n).collect();
        let size = vec![1; n + 1];
        let n_comp = n;
        Self { parent, size, n_comp }
    }
    
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb { return; }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.n_comp -= 1;
    }
}

mod solution {
    use super::*;
    pub fn solve() {
        let stdin = io::stdin();
        let mut sc = Scanner::new(stdin.lock());

        let n = sc.next();
        let p = (0..=n).map(|i| if i == 0 { 0 } else { sc.next()}).collect::<Vec<usize>>();
        let mut dsu = DSU::new(n);
        for i in 1..=n {
            dsu.union(i, p[i]);
        } 

        println!("{}", dsu.n_comp);
    }
}



use std::io::{self, BufRead};
// use std::str::FromStr;

pub struct Scanner<R> {
    reader: R,
    buf: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(s) = self.buf.pop() {
                return s.parse().ok().unwrap();
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).unwrap();
            self.buf = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}