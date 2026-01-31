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
    let (n, q): (usize, usize) = (sc.next(), sc.next());

    let mut prf = vec![vec![0_usize; n + 1]; n + 1];

    for i in 1..=n {
        let line: String = sc.next();
        let line_bytes = line.as_bytes();
        for j in 1..=n {
            prf[i][j] = prf[i - 1][j] + prf[i][j - 1] - prf[i - 1][j - 1];
            if line_bytes[j - 1] == b'*' {
                prf[i][j] += 1;
            }
        }
    }

    for _ in 0..q {
        let (ax, ay): (usize, usize) = (sc.next(), sc.next());
        let (bx, by): (usize, usize) = (sc.next(), sc.next());
        let answer = prf[bx][by] - prf[bx][ay - 1] - prf[ax - 1][by] + prf[ax - 1][ay - 1];
        writeln!(out, "{}", answer).ok();
    }
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
