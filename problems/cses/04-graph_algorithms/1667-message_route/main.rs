use std::io::{self, Read};
use std::collections::VecDeque;

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
        self.iter.next().expect("Unexpected end of file.")
            .parse().ok().expect("Cannot parse the input.")
    }
}

#[derive(Debug)]
struct Graph {
    num_nodes: usize, 
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            adj_list: vec![vec![]; num_nodes + 1],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }
}

fn solve(graph: &Graph) {
    let start = 1;
    let end = graph.num_nodes;

    let mut parent = vec![0; graph.num_nodes + 1];
    let mut queue  = VecDeque::new();

    queue.push_back(start);
    parent[start] = start;

    let mut found = false;

    while let Some(u) = queue.pop_front() {
        if u == end {
            found = true;
            break;
        }

        for &v in &graph.adj_list[u] {
            if parent[v] == 0 {
                parent[v] = u;
                queue.push_back(v);
            }
        }

    }
    if !found {
        println!("IMPOSSIBLE");
        return 
    }
    let mut path = Vec::with_capacity(graph.num_nodes);
    let mut curr = end;
    while curr != start {
        path.push(curr);
        curr = parent[curr];
    }
    path.push(start);

    println!("{}", path.len());
    for (_, node) in path.iter().rev().enumerate() {
        print!("{} ", node);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Cannot read from stdin");
    let mut sc = Scanner::new(&input);

    let num_nodes: usize = sc.next();
    let num_edges: usize = sc.next();

    let mut graph = Graph::new(num_nodes);
    
    for _ in 0..num_edges {
        let src: usize = sc.next();
        let dst: usize = sc.next();

        graph.add_edge(src, dst);
    }
    // println!("{:?}", graph);
    solve(&graph);
}
