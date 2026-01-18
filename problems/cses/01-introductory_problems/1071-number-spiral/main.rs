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
        self.iter.next().expect("Unexpected end of file")
            .parse().ok().expect("Cannot read the file")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);

    let num_queries: usize = sc.next();
    for _ in 0..num_queries {
        let x: i64 = sc.next();
        let y: i64 = sc.next();

        let layer: i64 = std::cmp::max(x, y);
        let middle_number = layer * (layer - 1) + 1;
         
        let phase = if layer % 2 == 0 { -1 } else { 1 };

        let step_from_middle = (x - y).abs();
        
        if x <= y {
            println!("{}", middle_number + phase * step_from_middle);
        }
        else {
            println!("{}", middle_number - phase * step_from_middle);
        }
    }
}