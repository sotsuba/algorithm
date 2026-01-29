use std::convert::TryFrom;
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

const INF: usize = usize::MAX;

fn compute_shortest_path(
    edge_list: &[(usize, usize, usize)],
    cost: &mut [Vec<usize>],
    num_nodes: usize,
) {
    for &(u, v, w) in edge_list {
        cost[u][v] = cost[u][v].min(w);
    }

    for i in 1..=num_nodes {
        cost[i][i] = 0;
    }

    for k in 1..=num_nodes {
        for j in 1..=num_nodes {
            for i in 1..=num_nodes {
                if cost[i][k] != INF && cost[k][j] != INF {
                    cost[i][j] = cost[i][j].min(cost[i][k] + cost[k][j]);
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);
    let num_nodes: usize = sc.next();
    let num_edges: usize = sc.next();
    let num_queries: usize = sc.next();

    let mut edge_list: Vec<(usize, usize, usize)> = Vec::with_capacity(num_edges + 1);

    for _ in 1..=num_edges {
        let src: usize = sc.next();
        let dst: usize = sc.next();
        let cost: usize = sc.next();
        edge_list.push((src, dst, cost));
        edge_list.push((dst, src, cost));
    }

    let mut costs = vec![vec![INF; num_nodes + 1]; num_nodes + 1];
    compute_shortest_path(&edge_list, &mut costs, num_nodes);

    for _ in 0..num_queries {
        let src: usize = sc.next();
        let dst: usize = sc.next();
        let answer = i64::try_from(costs[src][dst]).unwrap_or(-1);
        println!("{}", answer);
    }
}
