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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);

    let output = io::stdout();
    let mut output = BufWriter::new(output.lock());

    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut prefix_sum = vec![0_usize; n + 1];

    for i in 1..=n {
        let value: usize = sc.next();
        prefix_sum[i] = prefix_sum[i - 1] ^ value;
    }

    for _ in 0..q {
        let l: usize = sc.next();
        let r: usize = sc.next();
        let answer = prefix_sum[r] ^ prefix_sum[l - 1];
        writeln!(output, "{}", answer).unwrap();
    }
}
