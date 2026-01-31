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

fn generate_permutation(
    curr: &mut String,
    len: usize,
    char_count: &mut [usize],
    perms: &mut Vec<String>,
) {
    if curr.len() == len {
        perms.push(curr.to_string());
        return;
    }

    for i in 0..26 {
        if char_count[i] > 0 {
            char_count[i] -= 1;
            let c = (b'a' + i as u8) as char;

            curr.push(c);
            generate_permutation(curr, len, char_count, perms);

            curr.pop();
            char_count[i] += 1;
        }
    }
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let s: String = sc.next();
    let mut curr = String::new();
    let mut char_count = [0_usize; 26];
    for &b in s.as_bytes() {
        char_count[(b - b'a') as usize] += 1;
    }

    let mut perms: Vec<String> = Vec::new();
    generate_permutation(&mut curr, s.len(), &mut char_count, &mut perms);
    writeln!(out, "{}", perms.len()).ok();
    for perm in &perms {
        writeln!(out, "{}", &perm).ok();
    }
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
