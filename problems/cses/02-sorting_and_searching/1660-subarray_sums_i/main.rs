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
    let k: usize = sc.next();

    let mut arr = vec![0_usize; n + 1];
    let mut pref = vec![0_usize; n + 1];

    for i in 1..=n {
        arr[i] = sc.next();
        pref[i] = pref[i - 1] + arr[i];
    }

    let mut answer = 0;
    for i in 0..=n {
        let expected = k + pref[i];
        match pref[i..].binary_search(&expected) {
            Ok(_) => answer += 1,
            _ => (),
        }
    }

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
