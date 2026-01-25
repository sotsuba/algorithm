impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let mut prefix = vec![vec![0; m + 1]; n + 1];
        
        for i in 1..=n {
            for j in 1..=m {
                prefix[i][j] = mat[i - 1][j - 1] + prefix[i][j - 1] + prefix[i - 1][j] - prefix[i - 1][j - 1];
            }
        }

        for size in (1..=n.min(m)).rev() {
            
            for i in size..=n {
                for j in size..=m {
                    if (prefix[i][j] - prefix[i][j - size] - prefix[i - size][j] + prefix[i - size][j - size]) <= threshold {
                        return size as i32;
                    }
                }
            }
        }
        0
    }
}