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
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);
    // ================== //

    let n: usize = sc.next();
    let mut data = vec![0; n + 1];
    let mut total = 0;
    for i in 1..=n {
        data[i] = sc.next();
        total += data[i];
    }

    let mut achivable_sum = vec![false; total + 1];
    achivable_sum[0] = true;

    for i in 1..=n {
        for sum in (data[i]..=total).rev() {
            if achivable_sum[sum - data[i]] == true {
                achivable_sum[sum] = true;
            }
        }
    }
    let num_achievable = achivable_sum.iter().filter(|&&b| b).count() - 1;
    println!("{}", num_achievable);
    for i in 1..=total {
        if achivable_sum[i] {
            print!("{} ", i);
        }
    }
}
