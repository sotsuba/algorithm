use std::io::{self, Read};
use std::str::FromStr;

const MOD: usize = 1_000_000_007;

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut nums = vec![0_usize; n + 1];
    for i in 1..=n {
        nums[i] = sc.next();
    }


    let mut dp = vec![vec![0_usize; m + 1]; n + 1];
    if nums[1] == 0 {
        for i in 1..=m {
            dp[1][i] = 1;
        }
    }
    else {
        dp[1][nums[1]] = 1;
    }

    for i in 2..=n {
        let left = if nums[i] == 0 { 1 } else { nums[i] };
        let right = if nums[i] == 0 { m } else { nums[i] };

        for num in left..=right {
            let a = dp[i - 1][num - 1];
            let b = dp[i - 1][num];
            let c = if num + 1 <= m { dp[i - 1][num + 1] } else { 0 };
            dp[i][num] = ((a + b) % MOD + c) % MOD;
        }
    }

    let mut ans = 0;
    for i in 1..=m {
        ans = (ans + dp[n][i]) % MOD;
    }
    println!("{}", ans);
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

