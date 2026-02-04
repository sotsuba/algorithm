impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut inc = vec![1_usize; n];
        let mut des = vec![1_usize; n];
        let mut num_inc = 0;
        let mut num_des = 0;

        for i in 1..n {
            if nums[i] == nums[i - 1] {
                return false;
            }
            inc[i] = 1 + if nums[i] > nums[i - 1] { inc[i - 1] } else { 0 };
            des[i] = 1 + if nums[i] < nums[i - 1] { des[i - 1] } else { 0 };
            if inc[i] == 2 {
                num_inc += 1;
            }
            if des[i] == 2 {
                num_des += 1;
            }
        }

        num_inc == 2 && num_des == 1
    }
}
