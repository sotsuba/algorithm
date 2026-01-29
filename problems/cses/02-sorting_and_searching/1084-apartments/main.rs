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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);

    let output = io::stdout();
    let mut output = BufWriter::new(output.lock());

    // ================== //

    let (n, m, range): (usize, usize, usize) = (sc.next(), sc.next(), sc.next());
    let mut desired_size: Vec<usize> = Vec::with_capacity(n);
    let mut department_size: Vec<usize> = Vec::with_capacity(m);

    for _ in 0..n {
        desired_size.push(sc.next());
    }

    for _ in 0..m {
        department_size.push(sc.next());
    }

    desired_size.sort_unstable();
    department_size.sort_unstable();

    let (mut i, mut j) = (0_usize, 0_usize);
    let mut answer = 0;

    while i < n && j < m {
        let left_bound = desired_size[i].saturating_sub(range);
        let right_bound = desired_size[i].saturating_add(range);
        if left_bound > department_size[j] {
            j += 1;
            continue;
        }
        if right_bound < department_size[j] {
            i += 1;
            continue;
        }

        if left_bound <= department_size[j] && department_size[j] <= right_bound {
            answer += 1;
            i += 1;
            j += 1;
        }
    }
    writeln!(output, "{}", answer).unwrap();
}
