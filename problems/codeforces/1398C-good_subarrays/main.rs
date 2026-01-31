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
    let mut arr = Vec::with_capacity(n + 1);
    arr.push(0);
    let s: String = sc.next();
    for c in s.as_bytes() {
        arr.push((c - b'0') as i32);
    }

    let mut pref = vec![0_i32; n + 1];
    let mut stack = vec![0_i32; n + 1];
    for i in 1..=n {
        pref[i] = pref[i - 1] + arr[i];
        stack[i] = pref[i] - (i as i32);
    }

    stack.sort_unstable();
    stack.push(i32::MAX);

    let mut ans = 0;
    let mut l = 0;
    for r in 1..=(n + 1) {
        if stack[r] != stack[l] {
            let x = r - l;
            ans += x * (x - 1) / 2;
            l = r;
        }
    }
    writeln!(out, "{}", ans);
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
