use std::io::{self, BufWriter, Read, Write};

struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}
impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            iter: input.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter
            .next()
            .expect("Unexpected end of file")
            .parse()
            .ok()
            .expect("Cannot read the file")
    }
}

fn main() {
    let input_data = load_input();
    let mut sc = Scanner::new(&input_data);

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    // let q: usize = sc.next();
    // for _ in 0..q {
    solve(&mut sc, &mut out);
    // }
}

const MOD: i32 = 1_000_000_007;

fn backtrack(
    mask: usize,
    curr: usize,
    n: usize,
    dp: &mut Vec<Vec<i32>>,
    adj: &Vec<Vec<usize>>,
) -> i32 {
    if curr == n - 1 {
        return if mask == (1 << n) - 1 { 1 } else { 0 };
    }

    if dp[mask][curr] != -1 {
        return dp[mask][curr];
    }

    let mut res = 0;

    for &next in &adj[curr] {
        if (mask >> next) & 1 == 0 {
            res = (res + backtrack(mask | (1 << next), next, n, dp, adj)) % MOD;
        }
    }
    dp[mask][curr] = res;
    dp[mask][curr]
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut adj = vec![Vec::new(); n];
    for _ in 0..m {
        let u = sc.next::<usize>() - 1;
        let v = sc.next::<usize>() - 1;
        adj[u].push(v);
    }
    let mut dp = vec![vec![-1_i32; n]; 1 << n];
    let answer = backtrack(1, 0, n, &mut dp, &adj);
    writeln!(out, "{}", answer).ok();
}

fn load_input() -> String {
    let mut input_data = String::new();
    #[cfg(debug_assertions)]
    {
        use std::fs::File;
        if let Ok(mut f) = File::open("data/input.txt") {
            f.read_to_string(&mut input_data).unwrap();
            return input_data;
        }
    }
    io::stdin().read_to_string(&mut input_data).unwrap();
    input_data
}
