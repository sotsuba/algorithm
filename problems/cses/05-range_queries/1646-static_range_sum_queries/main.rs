use std::io::{self, Read};

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
    io::stdin()
        .read_to_string(&mut input)
        .expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut arr = vec![0_usize; n + 1];
    let mut prefix_sum = vec![0_usize; n + 1];
    prefix_sum[0] = 0;
    for i in 1..=n {
        arr[i] = sc.next();
        prefix_sum[i] = prefix_sum[i - 1] + arr[i];
    }

    for _ in 0..q {
        let l: usize = sc.next();
        let r: usize = sc.next();

        println!("{}", prefix_sum[r] - prefix_sum[l - 1]);
    }
}
