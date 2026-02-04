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

    let q: usize = sc.next();
    for _ in 0..q {
        solve(&mut sc, &mut out);
    }
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: usize = sc.next();
    let mut arr: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(sc.next());
    }
    let mut max_pref = vec![i64::MIN; n];
    let mut max_suff = vec![i64::MIN; n];
    max_pref[0] = arr[0];
    for i in 1..n {
        max_pref[i] = max_pref[i - 1].max(arr[i] + i as i64);
    }

    max_suff[n - 1] = arr[n - 1] - (n - 1) as i64;
    for i in (0..n - 1).rev() {
        max_suff[i] = max_suff[i + 1].max(arr[i] - i as i64);
    }

    let mut ans = i64::MIN;
    for i in 1..(n - 1) {
        ans = ans.max(max_pref[i - 1] + max_suff[i + 1] + arr[i]);
    }

    writeln!(out, "{}", ans).ok();
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
