fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let t: usize = 1;
    for _ in 0..t { 
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
    }
}

mod solution {
    use super::*;
    const MOD: usize = 1_000_000_007;

    fn dp(mask: usize, last_song: usize, n: usize, genre: &[usize], cache: &mut [Vec<usize>]) -> usize {
        if mask == 0 { return 1; }

        if last_song != n && cache[mask][last_song] != usize::MAX { return cache[mask][last_song]; }

        let mut ans = 0;
        for next_song in 0..n {
            if (mask >> next_song) & 1 == 0 { continue; }  
            if last_song != n && genre[last_song] == genre[next_song] { continue; }

            let next_mask = mask ^ (1 << next_song);

            ans += dp(next_mask, next_song, n, genre, cache);
            ans %= MOD;
        }
        
        cache[mask][last_song] = ans;
        
        ans 
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, t, duration, genre } = tc;
        
        let total_masks: usize = (1 << n) - 1;
        
        let mut total_duration = vec![0_usize; 1 << n];
        for mask in 1..=total_masks {
            let bit = mask & mask.wrapping_neg();
            let i   = bit.trailing_zeros() as usize;
            total_duration[mask] = total_duration[mask ^ bit] + duration[i];
        }

        let mut cache = vec![vec![usize::MAX; n + 1]; 1 << n];

        let mut ans = 0;
        for mask in 0..=total_masks {
            if total_duration[mask] != t { continue; }
            ans += dp(mask, n, n, &genre, &mut cache);
            ans %= MOD;
        }
        println!("{}", ans);
    }
}

pub struct TestCase {
    n: usize,
    t: usize, 
    duration: Vec<usize>,
    genre: Vec<usize>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let t = sc.next();
        let (duration, genre) = (0..n).map(|_| (sc.next::<usize>(), sc.next::<usize>() - 1)).unzip();
        Self { n, t, duration, genre }
    }
}











use std::io::{self, Read};
use std::str::FromStr;

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

pub struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Scanner { iter: input.split_whitespace() }
    }
    fn next<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}