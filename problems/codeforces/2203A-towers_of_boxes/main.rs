fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let mut q: usize = sc.next();
    while q > 0 {
        let (n, m, d): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
        let box_per_tower = 1 + d / m;
        
        let mut tower = 0;
        let mut boxe = 0;
        while boxe < n {
            tower += 1;
            boxe += box_per_tower;
        }
        println!("{}", tower);
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

