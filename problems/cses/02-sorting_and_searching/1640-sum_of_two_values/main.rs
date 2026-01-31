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

    let mut data: Vec<(usize, usize)> = Vec::with_capacity(n);
    for i in 1..=n {
        data.push((sc.next(), i));
    }

    data.sort_unstable();
    for i in 0..n {
        if k < data[i].0 {
            continue;
        }
        let expected = k - data[i].0;
        if let Ok(j) = data.binary_search_by_key(&expected, |&(val, _)| val) {
            if i != j {
                writeln!(out, "{} {}", data[i].1, data[j].1).ok();
                return;
            } else if j + 1 < n && data[j + 1].0 == expected {
                writeln!(out, "{} {}", data[i].1, data[j + 1].1).ok();
                return;
            }
        }
    }
    writeln!(out, "IMPOSSIBLE");
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
