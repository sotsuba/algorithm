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

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let (n, m, k): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut arr = vec![0_i64; n + 1];
    for i in 1..=n {
        arr[i] = sc.next();
    }

    let mut operations = vec![(0_usize, 0_usize, 0_i64); m + 1];
    for i in 1..=m {
        operations[i] = (sc.next(), sc.next(), sc.next());
    }

    let mut diff_operations = vec![(0_i64); m + 2];
    for _ in 0..k {
        let (x, y): (usize, usize) = (sc.next(), sc.next());
        diff_operations[x] += 1;
        diff_operations[y + 1] -= 1;
    }

    for i in 1..=m {
        diff_operations[i] += diff_operations[i - 1];
    }

    let mut diff_arr = vec![0_i64; n + 2];
    for i in 1..=m {
        let (x, y, d) = operations[i];
        diff_arr[x] += d * diff_operations[i];
        diff_arr[y + 1] -= d * diff_operations[i];
    }
    for i in 1..=n {
        diff_arr[i] += diff_arr[i - 1];
    }

    for i in 1..=n {
        write!(out, "{} ", arr[i] + diff_arr[i]);
    }
    writeln!(out);
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
