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

    solve(&mut sc, &mut out);
}

const MOD: usize = 10_usize.pow(9) + 7;

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: usize = sc.next();

    let sum = n * (n + 1) / 2;
    if sum % 2 == 1 {
        writeln!(out, "0").ok();
        return;
    }
    let half_sum = sum / 2;

    let mut dp = vec![0_usize; half_sum + 1];
    dp[0] = 1;
    for i in 1..n {
        for cur in (i..=half_sum).rev() {
            dp[cur] = (dp[cur] + dp[cur - i]) % MOD;
        }
    }
    writeln!(out, "{}", dp[half_sum]).ok();
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
