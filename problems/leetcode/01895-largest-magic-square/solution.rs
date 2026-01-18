impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut rows, mut cols, mut d1, mut d2)= (
            vec![vec![0; n + 1]; m],
            vec![vec![0; n]; m + 1],
            vec![vec![0; n + 1]; m + 1],
            vec![vec![0; n + 1]; m + 1],
        );

        for i in 0..m {
            for j in 0..n {
                rows[i][j+1] = grid[i][j] + rows[i][j];
                cols[i+1][j] = grid[i][j] + cols[i][j];
                d1[i+1][j+1] = grid[i][j] + d1[i][j];
                d2[i+1][n-j-1] = grid[i][n-j-1] + d2[i][n-j];
            }
        }

        let is_magic = |i1: usize, j1: usize, i2: usize, j2: usize, print: bool| -> bool {
            let diag1 = d1[i2 + 1][j2 + 1] - d1[i1][j1];
            let diag2 = d2[i2 + 1][j1] - d2[i1][j2 + 1];
            diag1 == diag2 && (
                (i1..=i2).all(|i| (rows[i][j2+1] - rows[i][j1]) == diag1) &&
                (j1..=j2).all(|j| (cols[i2+1][j] - cols[i1][j]) == diag1)
            )
        };

        let mut ans = 1;
        for i in 0..m {
            for j in 0..n {
                for side in (2..=1+i.min(j)).rev() {
                    if is_magic(i-side+1, j-side+1, i, j, side == 3) {
                        ans = ans.max(side);
                        break;
                    }
                }
            }
        }
        ans as _
    }
}