use std::io::{self, Read}; 

#[derive(Debug)] 
struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
} 

impl <'a> Scanner<'a> {
    fn new (input: &'a str) -> Self {
        Self {
            iter: input.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().expect("Unexpected end of input.")
            .parse()
            .ok().expect("Failed to parse token")
    }
}



#[derive(Debug)]
pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            size:  vec![1; n + 1],
        }
    }

    pub fn get(&mut self, mut vertex: usize) -> usize {
        if self.parent[vertex] == vertex {
            vertex
        } else {
            self.parent[vertex] = self.get(self.parent[vertex]);
            self.parent[vertex]
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root_a: usize = self.get(a);
        let root_b: usize = self.get(b);
        
        if root_a != root_b {
            if self.size[root_a] < self.size[root_b] {
                self.parent[root_a] = root_b;
                self.size[root_b] += self.size[root_a];
            }
            else {
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b];
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");

    let mut sc = Scanner::new(&input);

    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut dsu = DSU::new(n);
    
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        dsu.union(u, v);
    }
    
    let mut roots = Vec::new();
    for i in 1..=n {
        if dsu.parent[i] == i {
            roots.push(i);
        }
    }

    // println!("{:?}", roots);
    // println!("{:?}", dsu);
    
    println!("{}", roots.len() - 1);

    for i in 0..roots.len() - 1 {
        println!("{} {}", roots[i], roots[i + 1]);
    }
}