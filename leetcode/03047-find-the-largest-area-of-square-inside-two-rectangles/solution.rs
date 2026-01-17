use std::cmp::{max, min};

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();
        let mut ans = 0_i64;

        for i in 0..n {
            for j in i+1..n {
                let bl = max(bottom_left[i][0], bottom_left[j][0]);
                let tr = min(top_right[i][0], top_right[j][0]);
                if bl >= tr {
                    continue 
                }
                
                let br = max(bottom_left[i][1], bottom_left[j][1]);
                let tl = min(top_right[i][1], top_right[j][1]);
                if br >= tl {
                    continue 
                }

                let width = min(tr - bl, tl - br) as i64;
                ans = max(ans, width * width);
            }
        }

        ans 
    }
}