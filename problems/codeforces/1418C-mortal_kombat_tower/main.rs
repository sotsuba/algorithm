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

const INF: usize = usize::MAX;

fn solve(sc: &mut Scanner) -> usize {
    let n: usize = sc.next();
    let mut data = vec![0_usize; n + 1];
    for i in 1..=n {
        data[i] = sc.next();
    }

    let mut dp = vec![[INF; 2]; n + 1];

    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][1] = dp[i][1].min(dp[i - 1][0].saturating_add(data[i]));
        dp[i][0] = dp[i][0].min(dp[i - 1][1]);
        if i >= 2 {
            dp[i][1] = dp[i][1].min(dp[i - 2][0].saturating_add(data[i] + data[i - 1]));
            dp[i][0] = dp[i][0].min(dp[i - 2][1]);
        }
    }
    dp[n][0].min(dp[n][1])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);

    let output = io::stdout();
    let mut output = BufWriter::new(output.lock());

    // ================== //

    let q: usize = sc.next();
    for _ in 0..q {
        let answer = solve(&mut sc);
        writeln!(output, "{}", answer).unwrap();
    }
}
