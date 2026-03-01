use std::io::{self, Read};
use std::str::FromStr;

const MOD: usize = 1_000_000_007;

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    
    let mut n: usize = sc.next();
    let mut nums = vec![0_usize; n];
    for i in 0..n {
        nums[i] = sc.next();
    }


    let m: usize = *nums.iter().max().unwrap();

    let mut dp = vec![vec![0_usize; 2]; m + 1];
    dp[1].fill(1);
    for i in 2..=m {
        dp[i][0] = (4 * dp[i - 1][0] + 1 * dp[i - 1][1]) % MOD; 
        dp[i][1] = (1 * dp[i - 1][0] + 2 * dp[i - 1][1]) % MOD; 
    }
    for num in nums {
        let ans = (dp[num][0] + dp[num][1]) % MOD;
        println!("{}", ans);
    }
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

