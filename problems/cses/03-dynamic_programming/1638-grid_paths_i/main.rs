use std::io::{self, Read};
use std::str::FromStr;

const MOD: usize = 1_000_000_007;

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let n: usize = sc.next();
    let mut mat = vec![vec![false; n + 1]; n + 1];
    for i in 0..n {
        let row: String = sc.next();

        for (j, b) in row.bytes().enumerate() {
            mat[i + 1][j + 1] = b == b'.';
        }
    }


    let mut dp = vec![vec![0_usize; n + 1]; n + 1];
    dp[1][1] = if mat[1][1] { 1 } else { 0 };

    for i in 1..=n {
        for j in 1..=n {
            if mat[i][j] == false { continue; }

            dp[i][j] += if mat[i][j - 1] { dp[i][j - 1] } else { 0 };
            dp[i][j] += if mat[i - 1][j] { dp[i - 1][j] } else { 0 };
            dp[i][j] %= MOD;
        }
    }

    println!("{}", dp[n][n]);
}









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

struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

