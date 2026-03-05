impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if k == 1 { return '0'; }
        let n = n as u32;
        let mut k = k as u32;

        let mut rev = 0;
        for i in (2..=n).rev() {
            let capa = 2_u32.pow(i) - 1;
            let mid = capa / 2 +ev % 2 == 1 { '0' } else { '1' };
            }
            else 
            if k > mid { 
                rev += 1;
                k = capa - k + 1;
            }
        }
        if rev % 2 == 0 { '0' } else { '1' }
    }
}