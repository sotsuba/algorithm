impl Solution {
    fn get_update(c: char) -> (usize, usize) {
        if c == '0' { return (0, 1); }
        else { return (1, 0); }
    }

    pub fn min_operations(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0_usize; n]; 2];
        for i in 0..n {
            let (u, v) = Self::get_update(s[i]);
            dp[0][i] = u;
            dp[1][i] = v;
            if i != 0 {
                dp[0][i] += dp[1][i - 1];
                dp[1][i] += dp[0][i - 1];
            }
        }
        let ans = dp[0][n - 1].min(dp[1][n - 1]);
        ans as i32
    }
}