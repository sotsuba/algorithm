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

#[derive(Debug)]
struct Rmq {
    block_size: usize,
    blocks: Vec<usize>,
    data: Vec<usize>,
}

impl Rmq {
    pub fn new(data: &[usize], data_len: usize, block_size: usize) -> Self {
        let num_block = data_len.div_ceil(block_size) + 1;
        let mut blocks = vec![0_usize; num_block];

        for i in 0..data_len {
            let id = i / block_size;
            blocks[id] += data[i];
        }

        Self {
            block_size,
            blocks,
            data: data.to_vec(),
        }
    }

    pub fn update(&mut self, index: usize, new_value: usize) {
        let id = self.compute_id(index);
        self.blocks[id] = self.blocks[id] - self.data[index] + new_value;
        self.data[index] = new_value;
    }

    pub fn return_sum_of_range(&self, left: usize, right: usize) -> usize {
        let left_id = self.compute_id(left);
        let right_id = self.compute_id(right);

        let mut answer = 0;

        if left_id == right_id {
            for i in left..=right {
                answer += self.data[i];
            }
        } else {
            for id in (left_id + 1)..right_id {
                answer += self.blocks[id];
            }

            for i in left..((left_id + 1) * self.block_size) {
                answer += self.data[i];
            }

            for i in (right_id * self.block_size)..=right {
                answer += self.data[i];
            }
        }

        answer
    }

    fn compute_id(&self, index: usize) -> usize {
        index / self.block_size
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

    let mut data = vec![0_usize; n];
    for i in 0..n {
        data[i] = sc.next();
    }

    let mut rmq = Rmq::new(&data, n, 1000);

    for _ in 0..q {
        let query_type: usize = sc.next();
        let a: usize = sc.next();
        let b: usize = sc.next();
        if query_type == 1 {
            rmq.update(a - 1, b);
        } else {
            let answer = rmq.return_sum_of_range(a - 1, b - 1);
            println!("{}", answer);
        }
    }
}
