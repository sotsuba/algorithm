use std::io::{self};

fn main() {
    // ================== //

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    const MOD: usize = 10_usize.pow(9) + 7;
    let mut dp = vec![0_usize; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for j in (i.saturating_sub(6))..i {
            dp[i] = (dp[i] + dp[j]) % MOD;
        }
    }
    println!("{}", dp[n]);
}
