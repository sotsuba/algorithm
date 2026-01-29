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

struct Rmq {
    block_size: usize,
    block_len: usize,
    block_min: Vec<usize>,
    n: usize,
    data: Vec<usize>,
}

const INF: usize = usize::MAX;

impl Rmq {
    pub fn new(block_size: usize, data: &[usize], n: usize) -> Self {
        let block_len = n.div_ceil(block_size);
        let mut block_min = vec![INF; block_len];
        for i in 1..=n {
            let id = i / block_size;
            block_min[id] = block_min[id].min(data[i]);
        }
        Self {
            block_size,
            block_len,
            block_min,
            n,
            data: data.to_vec(),
        }
    }

    fn query(&self, left_bound: usize, right_bound: usize) -> usize {
        let left_block_id = left_bound.div_ceil(self.block_size);
        let right_block_id = right_bound / self.block_size;
        let remain_right = right_bound % self.block_size;
        // let remain_left = self.block_size * left_block_id - left_bound - 1;
        let mut ans = INF;
        // for i in left_block_id..right_block_id {
        //     ans = ans.min(self.block_min[i]);
        // }

        // for i in (right_bound - remain_right)..=right_bound {
        //     ans = ans.min(self.data[i]);
        // }

        // for i in left_bound..=(left_bound + remain_left) {
        //     ans = ans.min(self.data[i]);
        // }
        println!("{} - {}", left_block_id, right_block_id);
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Cannot read the stdin");

    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let q: usize = sc.next();

    let mut arr = vec![0_usize; n + 1];
    for i in 1..=n {
        arr[i] = sc.next();
    }
    println!("{:?}", arr);
    let rmq = Rmq::new(3, &arr, n);
    println!("{:?}", rmq.block_min);
    for _ in 0..q {
        let l: usize = sc.next();
        let r: usize = sc.next();

        rmq.query(l, r);
    }
}
