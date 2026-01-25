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
/*
My solution:
- Find the less significant of unset bit (let's say LSUB), because from here, ans + 1 takes no effect.
- Base on LSUB, unset the bit next to it.

*/