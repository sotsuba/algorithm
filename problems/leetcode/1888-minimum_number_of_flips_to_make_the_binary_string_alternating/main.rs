impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let doubled: Vec<char> = s.chars().chain(s.chars()).collect();
        let m = 2 * n;

        let mut pref0 = vec![0usize; m + 1]; 
        let mut pref1 = vec![0usize; m + 1]; 

        for i in 0..m {
            let expected0 = if i % 2 == 0 { '0' } else { '1' };
            let expected1 = if i % 2 == 0 { '1' } else { '0' };

            pref0[i + 1] = pref0[i] + (doubled[i] != expected0) as usize;
            pref1[i + 1] = pref1[i] + (doubled[i] != expected1) as usize;
        }

        let mut ans = usize::MAX;
        for l in 0..=m - n {
            let r = l + n;
            let cost0 = pref0[r] - pref0[l];
            let cost1 = pref1[r] - pref1[l];
            ans = ans.min(cost0.min(cost1));
        }

        ans as i32
    }
}