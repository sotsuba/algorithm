use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);
    
    let n: usize = sc.next();
    let m: usize = sc.next();
    let nums1: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    let nums2: Vec<usize> = (0..m).map(|_| sc.next()).collect();


    let mut dp = vec![vec![0_usize; m + 1]; n + 1];
    let mut max_dp = vec![vec![0_usize; m + 1]; n + 1];
    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=m {
            let inc = if nums1[i - 1] == nums2[j - 1] { 1 } else { 0 };

            dp[i][j] = max_dp[i - 1][j - 1] + inc;
            
            max_dp[i][j] = *[max_dp[i - 1][j], max_dp[i][j - 1], dp[i][j]].iter().max().unwrap();
                
            ans = ans.max(dp[i][j]);
        }
    }

    println!("{}", ans);
    let mut collected = Vec::with_capacity(ans);
    
    let mut i = n;
    let mut k = m;
    let mut tmp = ans;
   
    while tmp > 0 {
        for j in (1..=k).rev() {
            if dp[i][j] == tmp && nums1[i - 1] == nums2[j - 1] {
                k -= 1;
                tmp -= 1;
                collected.push(nums1[i - 1]);
                break;
            }
        }     
        i -= 1;
    }

    for i in (0..ans).rev() {
        print!("{} ", collected[i]);
    }
    println!();
}   











fn adapted_input() -> String {
    match cfg!(debug_assertions) {
        true => std::fs::read_to_string("input.txt").unwrap(),
        false => { 
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    }
}

struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

