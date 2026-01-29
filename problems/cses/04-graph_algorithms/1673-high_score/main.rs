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

const INF: i64 = 1_000_000_000_000_000;
const NINF: i64 = -1_000_000_000_000_000;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut edges = Vec::with_capacity(m + 1);

    for _ in 0..m {
        let src: usize = sc.next();
        let dst: usize = sc.next();
        let cost: i64 = sc.next();
        edges.push((src, dst, cost));
    }

    let mut dist = vec![NINF; n + 1];
    dist[1] = 0;

    for _ in 0..n - 1 {
        for &(u, v, w) in &edges {
            if dist[u] != NINF {
                dist[v] = dist[v].max(dist[u] + w);
            }
        }
    }

    for _ in 0..n {
        for &(u, v, w) in &edges {
            if dist[u] != NINF {
                if dist[u] == INF {
                    dist[v] = INF;
                } else if dist[u] + w > dist[v] {
                    dist[v] = INF;
                }
            }
        }
    }

    if dist[n] == INF {
        println!("-1");
    } else {
        println!("{}", dist[n]);
    }
}
