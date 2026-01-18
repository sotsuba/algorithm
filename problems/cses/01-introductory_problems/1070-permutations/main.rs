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
        self.iter.next().expect("Unexpected end of file.")
            .parse().ok().expect("Cannot parse the input.")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Cannot read from stdin");
    let mut sc = Scanner::new(&input);

    let n = sc.next();
    
    if n == 2 || n == 3 {
        println!("NO SOLUTION");
        return 
    }
    
    for i in (2..=n).step_by(2) {
        print!("{} ", i);
    }

    for i in (1..=n).step_by(2) {
        print!("{} ", i);
    }

    println!();
}
