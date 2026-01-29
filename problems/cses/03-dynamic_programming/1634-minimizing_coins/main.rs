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
    let target: usize = sc.next();
    let mut data = vec![0; n + 1];
    for i in 1..=n {
        data[i] = sc.next();
    }

    let mut achivable_sum = vec![usize::MAX; target + 1];
    achivable_sum[0] = 0;

    for i in 1..=n {
        for sum in data[i]..=target {
            if achivable_sum[sum - data[i]] == usize::MAX {
                continue;
            }
            achivable_sum[sum] = achivable_sum[sum].min(achivable_sum[sum - data[i]] + 1);
        }
    }
    if achivable_sum[target] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", achivable_sum[target]);
    }
}
