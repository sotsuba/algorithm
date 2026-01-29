use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

fn dijkstra(adj: &[Vec<(usize, usize)>], num_nodes: usize) -> usize {
    let mut dist = vec![[usize::MAX; 2]; num_nodes + 1];
    let src = 1;
    let dst = num_nodes;
    dist[src][0] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, src, false)));

    while let Some(Reverse((d, u, is_discounted))) = pq.pop() {
        if d > dist[u][is_discounted as usize] {
            continue;
        }

        for &(v, w) in &adj[u] {
            if !is_discounted {
                let new_dist = dist[u][0] + w / 2;
                if new_dist < dist[v][1] {
                    dist[v][1] = new_dist;
                    pq.push(Reverse((new_dist, v, true)));
                }
            }

            let new_dist = dist[u][is_discounted as usize] + w;
            if new_dist < dist[v][is_discounted as usize] {
                dist[v][is_discounted as usize] = new_dist;
                pq.push(Reverse((new_dist, v, is_discounted)));
            }
        }
    }
    std::cmp::min(dist[dst][0], dist[dst][1])
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: usize = sc.next();
        adj[u].push((v, w));
    }

    println!("{}", dijkstra(&adj, n));
}
