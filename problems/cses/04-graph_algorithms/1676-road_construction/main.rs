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

    fn next<T: std::str::FromStr> (&mut self) -> T {
        self.iter.next().expect("Unexpected end of file")
            .parse::<T>().ok().expect("Cannot parse the token")
    }
}

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: u32,
    largest_component: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            num_components: n as u32,
            largest_component: 1,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
            return self.parent[i]
        }
        i 
    }

    fn unite(&mut self, a: usize, b: usize) {
        let root_a = self.find(a); 
        let root_b = self.find(b);

        if root_a != root_b {
            if self.size[root_a] >= self.size[root_b] {
                self.size[root_a] += self.size[root_b];
                self.parent[root_b] = self.parent[root_a];
                self.largest_component = std::cmp::max(self.largest_component, self.size[root_a]);
            }
            else {
                self.size[root_b] += self.size[root_a];
                self.parent[root_a] = self.parent[root_b];
                self.largest_component = std::cmp::max(self.largest_component, self.size[root_b]);
            }
            self.num_components -= 1;
        } 

    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Cannot read from stdin");
    let mut sc = Scanner::new(&buffer);

    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut dsu = Dsu::new(n);
    
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        dsu.unite(u, v);
        println!("{} {}", dsu.num_components, dsu.largest_component);
    }
}