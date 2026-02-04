impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut dp = vec![vec![i64::MIN; n]; 3];

        let mut ans = i64::MIN;

        for i in 1..n {
            let prev = nums[i - 1] as i64;
            let curr = nums[i] as i64;

            // phase 0 inc
            if curr > prev {
                let extend = if dp[0][i - 1] == i64::MIN {
                    i64::MIN
                } else {
                    dp[0][i - 1] + curr
                };
                let switch = prev + curr;
                dp[0][i] = extend.max(switch);
            }

            // phase 1 dec
            if curr < prev {
                let extend = if dp[1][i - 1] == i64::MIN {
                    i64::MIN
                } else {
                    dp[1][i - 1] + curr
                };
                let switch = if dp[0][i - 1] == i64::MIN {
                    i64::MIN
                } else {
                    dp[0][i - 1] + curr
                };
                dp[1][i] = extend.max(switch);
            }

            // phase 2 dec
            if curr > prev {
                let extend = if dp[2][i - 1] == i64::MIN {
                    i64::MIN
                } else {
                    dp[2][i - 1] + curr
                };
                let switch = if dp[1][i - 1] == i64::MIN {
                    i64::MIN
                } else {
                    dp[1][i - 1] + curr
                };
                dp[2][i] = extend.max(switch);
            }

            if dp[2][i] != i64::MIN {
                ans = ans.max(dp[2][i]);
            }
        }

        if ans == i64::MIN {
            0
        } else {
            ans
        }
    }
}
