impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut idx = -1_i32;
        for (i, c) in s.chars().enumerate() {
            if c == '0' { continue; }

            if i as i32 > idx + 1 && idx != -1 {
                return false;
            }
            idx = i as i32;
        }
        true
    }
}