fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let mut q: usize = sc.next();
    while q > 0 {
        let s = sc.next::<String>();
        if s.len() <= 10 {
            println!("{}", s);
        }
        else {
            let mut iter = s.chars();
            let first_char = iter.next().unwrap();
            let last_char = iter.next_back().unwrap();
            println!("{}{}{}", first_char, s.len() - 2, last_char);
        }
        q -= 1;
    }
}   








































use std::io::{self, Read};
use std::str::FromStr;

fn adapted_input() -> String {
    match cfg!(debug_assertions) {
        true => std::fs::read_to_string("input.txt").unwrap(),
        false => { 
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    }
}

struct Scanner<'a> { iter: std::str::SplitWhitespace<'a> }

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self { Scanner { iter: input.split_whitespace() } }
    fn next<T: FromStr>(&mut self) -> T { self.iter.next().unwrap().parse().ok().unwrap() }
}

