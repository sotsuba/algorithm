use std::collections::HashMap;
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
    let input_data = load_input();
    let mut sc = Scanner::new(&input_data);

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    solve(&mut sc, &mut out);
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let n: i64 = sc.next();

    let mut map = HashMap::with_capacity(n as usize);
    map.insert(0, 1);

    let mut current_prf: i64 = 0;
    let mut answer: i64 = 0;

    for _ in 0..n {
        let val: i32 = sc.next();
        current_prf += val as i64;

        let target = current_prf.rem_euclid(n);
        if let Some(&count) = map.get(&target) {
            answer += count;
        }

        *map.entry(target).or_insert(0) += 1;
    }
    writeln!(out, "{}", answer).unwrap();
}

fn load_input() -> String {
    let mut input_data = String::new();
    #[cfg(debug_assertions)]
    {
        use std::fs::File;
        if let Ok(mut f) = File::open("data/input.txt") {
            f.read_to_string(&mut input_data).unwrap();
            return input_data;
        }
    }
    io::stdin().read_to_string(&mut input_data).unwrap();
    input_data
}
