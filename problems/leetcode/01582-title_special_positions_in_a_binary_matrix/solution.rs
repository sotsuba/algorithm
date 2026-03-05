impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut cnt_up = vec![0_usize; n];
        let mut cnt_side = vec![0_usize; m];
        for i in 0..n {
            for j in 0..m {
                cnt_up[i] += mat[i][j] as usize;
                cnt_side[j] += mat[i][j] as usize;
            }
        }
        println!("{:?}", &cnt_up);
        println!("{:?}", &cnt_side);
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += if mat[i][j] == 1 && cnt_up[i] == 1 && cnt_side[j] == 1 { 1 } else { 0 };
            }
        }

        ans
    }
}