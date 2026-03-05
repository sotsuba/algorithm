pub struct BIT {
    n: usize,
    bit: Vec<usize>,
}

impl BIT {
    pub fn new(n: usize) -> Self { Self { n, bit: vec![0_usize; n + 1] } }

    pub fn update(&mut self, idx: usize, val: usize) {
        let mut idx = idx;
        while idx <= self.n {
            self.bit[idx] = self.bit[idx].max(val);
            idx += idx & (!idx + 1);
        }
    }

    pub fn query(&self, idx: usize) -> usize {
        let mut idx = idx;

        let mut ans = 0;
        while idx > 0 {
            ans = ans.max(self.bit[idx]);
            idx &= idx - 1;
        }
        ans
    }
}

#[derive(Debug)]
pub struct Project {
    start: usize,
    end: usize,
    reward: usize,
}

pub struct TestCase {
    n: usize, 
    projects: Vec<Project>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let projects = (0..n).map( |_| Project { start: sc.next(), end: sc.next(), reward: sc.next()} ).collect();
        Self { n, projects }
    }
}

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}   


mod solution {
    use super::*;
    pub fn solve(tc: TestCase) {
        let n = tc.n;
        let mut projects = tc.projects;

        projects.sort_by_key(|o| o.start);
        
        let mut vals: Vec<usize> = projects.iter().flat_map(|o| [o.start, o.end]).collect();
        vals.sort_unstable();
        vals.dedup();

        let mut bit = BIT::new(vals.len());
        let mut ans = projects[0].reward;
        for i in 0..n {
            let start_idx = vals.binary_search(&projects[i].start).unwrap() + 1;
            let end_idx = vals.binary_search(&projects[i].end).unwrap() + 1;
            let val = bit.query(start_idx - 1) + projects[i].reward;
            bit.update(end_idx, val);
            ans = ans.max(val);
        }

        println!("{}", ans);
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

pub struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

