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

struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(data: &[usize]) -> Self {
        let n = data.len();
        let mut tree = vec![0; 2 * n];

        for i in 0..n {
            tree[n + i] = data[i];
        }

        for i in (1..n).rev() {
            tree[i] = tree[2 * i].min(tree[2 * i + 1]);
        }

        Self { n, tree }
    }

    fn update(&mut self, mut idx: usize, value: usize) {
        idx += self.n;
        self.tree[idx] = value;

        while idx > 1 {
            idx /= 2;
            self.tree[idx] = self.tree[2 * idx].min(self.tree[2 * idx + 1]);
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> usize {
        l += self.n;
        r += self.n;
        let mut answer = usize::MAX;

        while l < r {
            if l % 2 == 1 {
                answer = answer.min(self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                answer = answer.min(self.tree[r]);
            }
            l /= 2;
            r /= 2;
        }
        answer
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(&input);

    let output = io::stdout();
    let mut output = BufWriter::new(output.lock());

    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        data.push(sc.next());
    }

    let mut tree = SegmentTree::new(&data);
    for _ in 0..q {
        let t: usize = sc.next();
        if t == 1 {
            let k: usize = sc.next();
            let u: usize = sc.next();
            tree.update(k - 1, u);
        } else {
            let a: usize = sc.next();
            let b: usize = sc.next();
            writeln!(output, "{}", tree.query(a - 1, b)).unwrap();
        }
    }
}
