impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut partial_cost = nums[1] + nums[2];
        for i in 1..n {
            for j in (i + 1)..n {
                partial_cost = partial_cost.min(nums[i] + nums[j]);
            }
        }
        partial_cost + nums[0]
    }
}
