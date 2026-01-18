use std::io::{self, Read};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone)]
struct Node {
    dest: usize,
    weight: usize,
}

#[derive(Debug)]
struct Graph {
    num_nodes: usize,
    adj_list: Vec<Vec<Node>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            adj_list: vec![vec![]; num_nodes + 1],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: usize) {
        self.adj_list[u].push(Node {dest: v, weight: w});
    }

    fn find_shortest_path_from(&self, source: usize) {
        let mut dist = vec![usize::MAX; self.num_nodes + 1];
        let mut heap = BinaryHeap::new();

        dist[source] = 0;
        heap.push(State { cost: 0, position: source });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > dist[position] {
                continue;
            }

            for edge in &self.adj_list[position] {
                let next_cost = cost + edge.weight;
                if next_cost < dist[edge.dest] {
                    dist[edge.dest] = next_cost;
                    heap.push(State {cost: next_cost, position: edge.dest });
                }
            }
        }

        for i in 1..=self.num_nodes {
            print!("{} ", dist[i]);
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);

    let num_nodes: usize = sc.next();
    let num_edges: usize = sc.next();
    let mut graph = Graph::new(num_nodes);

    for _ in 0..num_edges {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: usize = sc.next();
        graph.add_edge(u, v, w);
    }
    graph.find_shortest_path_from(1);
}