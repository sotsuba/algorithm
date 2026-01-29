use std::io::{self, Read};

struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            iter: input.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter
            .next()
            .expect("Unexpected end of file")
            .parse()
            .ok()
            .expect("Cannot read the file")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);
    // ================== //

    let n: usize = sc.next();
    let expected: usize = sc.next();
    let mut coins = vec![0_usize; n + 1];
    for i in 1..=n {
        coins[i] = sc.next();
    }

    const MOD: usize = 10_usize.pow(9) + 7;
    let mut dp = vec![0_usize; expected + 1];
    dp[0] = 1;
    for j in 1..=expected {
        for i in 1..=n {
            if j < coins[i] {
                continue;
            }
            dp[j] = (dp[j] + dp[j - coins[i]]) % MOD;
        }
    }
    println!("{}", dp[expected]);
}
