impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let into_set = |mut f: Vec<_>, m: _| -> std::collections::BinaryHeap<_> {
            f.sort_unstable();
            std::iter::once(&1)
                .chain(&f)
                .enumerate()
                .flat_map(|(i, l)| f[i..].iter().chain([&m]).map(move |r| r - l))
                .collect()
        };
        let mut h = into_set(h_fences, m);
        let mut v = into_set(v_fences, n);
        while let (Some(hv), Some(vv)) = (h.peek(), v.peek()) {
            match hv.cmp(vv) {
                std::cmp::Ordering::Less => {
                    v.pop();
                }
                std::cmp::Ordering::Greater => {
                    h.pop();
                }
                std::cmp::Ordering::Equal => return (i64::from(*hv).pow(2) % 1_000_000_007) as _,
            }
        }
        -1
    }
}