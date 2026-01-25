impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
            nums.into_iter()
                .map(|n| {
                    if n % 2 == 0 { -1 }
                    else { n ^ ((!n & -!n) >> 1)}
                })
                .collect()
    }
}