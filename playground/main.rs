use std::io::{self, Read};

#[derive(Debug)]
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
            .next().expect("Unexpected end of file")
            .parse().ok().expect("Cannot parse the token")
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("Cannot read from stdin");
    
}