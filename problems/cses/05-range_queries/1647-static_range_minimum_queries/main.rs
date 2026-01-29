use std::io::{self, Read};

struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Cannot read the stdin");
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

struct Rmq {
    block_size: usize,
    block_min: Vec<usize>,
    data: Vec<usize>,
}

const INF: usize = usize::MAX;

fn main() {
    let mut sc = Scanner::new);
    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut arr = vec![0_usize; n + 1];
    for i in 1..=n {
        arr[i] = sc.next();
    }
}
