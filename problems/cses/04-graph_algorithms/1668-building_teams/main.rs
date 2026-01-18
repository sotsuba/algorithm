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
        self.iter.next().expect("Unexpected end of file")
            .parse().ok().expect("Cannot read the file")
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

    fn add_node(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }
}

fn solve(graph: &Graph) -> Option<Vec<i8>> {
    let mut colors = vec![0_i8; graph.num_nodes + 1];

    for i in 1..=graph.num_nodes {
        if colors[i] == 0 {
            if !dfs_color(&graph, i, 1, &mut colors) {
                return None 
            }
        }
    }
    Some(colors)
}


fn dfs_color(graph: &Graph, u: usize, c: i8, colors: &mut Vec<i8>) -> bool {
    colors[u] = c;
    let next_color = 3 - c;

    for &v in &graph.adj_list[u] {
        if colors[v] == 0 {
            if !dfs_color(&graph, v, next_color, colors) {
                return false;
            }
        } else if colors[v] == c {
            return false;
        }
    }
    true 
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
        graph.add_node(u, v);
    }

    match solve(&graph) {
        Some (colors) => {
            for i in 1..=graph.num_nodes {
                print!("{} ", colors[i]);
            };
            println!();
        },
        None => println!("IMPOSSIBLE"),
    }
}