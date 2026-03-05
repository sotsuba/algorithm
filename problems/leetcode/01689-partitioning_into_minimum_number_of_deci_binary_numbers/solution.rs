impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut cnt: Vec<i8> = n.bytes().map(|x| (x - b'0') as i8).collect();   
        let n = cnt.len();
        println!("{:?}", &cnt);
        let mut ans: i32 = 0;
        for i in 0..n {
            if cnt[i] == 0 { continue; }
            ans += cnt[i] as i32;
            for j in (i..n).rev() {
                cnt[j] = (cnt[j] - cnt[i]).max(0);
            }
        }
        ans
    }
}