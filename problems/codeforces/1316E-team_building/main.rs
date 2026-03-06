fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    let tc = TestCase::new(&mut sc);
    solution::solve(tc);
}

mod solution {
    use super::*;
    const NEG_INF: i64 = -(1i64 << 60);

    pub fn solve(tc: TestCase) {
        let TestCase { n, p, k, a, s } = tc;
        let mut paired: Vec<_> = a.into_iter().zip(s.into_iter()).collect();
        paired.sort_by_key(|(ai, _)| std::cmp::Reverse(*ai));

        let (a, s): (Vec<_>, Vec<_>) = paired.into_iter().unzip();

        let size = 1 << p;
        let mut dp = vec![NEG_INF; size];
        dp[0] = 0;

        for i in 0..n {
            let mut ndp = dp.clone();
            
            for mask in 0..size {
                if dp[mask] == NEG_INF { continue; }

                let cnt = mask.count_ones() as usize;

                if i >= cnt && i - cnt < k {
                    ndp[mask] = ndp[mask].max(dp[mask] + a[i]);
                }

                for j in 0..p {
                    if (mask >> j) & 1 == 0 {
                        let new_mask = mask ^ (1 << j);
                        ndp[new_mask] = ndp[new_mask].max(dp[mask] + s[i][j]);
                    }
                }

            }
            dp = ndp;
        }

        println!("{}", dp[size - 1]);
    }
}

pub struct TestCase {
    n: usize, // num people
    p: usize, // num players
    k: usize, // num audience
    a: Vec<i64>, // value of audience.
    s: Vec<Vec<i64>> // value of players for the j position.
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let p = sc.next();
        let k = sc.next();
        let a: Vec<i64> = (0..n).map(|_| sc.next()).collect();
        let s: Vec<Vec<i64>> = (0..n).map(|_| {
            let row: Vec<i64> = (0..p).map(|_| sc.next()).collect();
            row
        }).collect();

        Self { n, p, k, a, s }

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