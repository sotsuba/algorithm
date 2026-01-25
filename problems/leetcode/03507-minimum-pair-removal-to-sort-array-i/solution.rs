impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut arr = nums;
        let mut steps = 0;

        while !arr.windows(2).all(|w| w[0] <= w[1]) {
            let mut min_sum = i32::MAX;
            let mut min_index = 0;

            for i in 0..arr.len() - 1 {
                let sum = arr[i] + arr[i+1];
                if sum < min_sum {
                    min_sum = sum;
                    min_index = i;
                }
            }

            arr[min_index] = min_sum;
            arr.remove(min_index + 1);
            
            steps += 1;
        }
        
        steps
    }
}