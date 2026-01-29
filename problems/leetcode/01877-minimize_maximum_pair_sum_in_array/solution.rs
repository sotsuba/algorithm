impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        nums.sort_unstable();

        let mut ans = 0;
        for i in 0..(n / 2) {
            ans = std::cmp::max(ans, nums[i] + nums[n - 1 - i]);
        }

        ans
    }
}
