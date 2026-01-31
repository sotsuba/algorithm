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

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: usize = sc.next();
    let mut arr: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(sc.next());
    }

    let mut ans = i64::MAX;

    for mask in 0..((1 << n) - 1) {
        let mut sub1 = 0;
        let mut sub2 = 0;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                sub1 += arr[i];
            } else {
                sub2 += arr[i];
            }
        }
        ans = ans.min((sub1 - sub2).abs());
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
