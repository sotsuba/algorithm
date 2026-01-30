use std::io::{self, BufWriter, Read, Write};

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

const ALPHABET_SIZE: usize = 26;
const MOD: i64 = 1_000_000_007;

#[derive(Debug, Default)]
struct Node {
    children: [Option<usize>; ALPHABET_SIZE],
    count: i64,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr_index = 0;
        for &byte in word.as_bytes() {
            let char_idx = (byte - b'a') as usize;

            if self.nodes[curr_index].children[char_idx].is_none() {
                let next_idx = self.create_node();
                self.nodes[curr_index].children[char_idx] = Some(next_idx);
            }

            curr_index = self.nodes[curr_index].children[char_idx].unwrap();
        }
        self.nodes[curr_index].count += 1;
    }

    fn create_node(&mut self) -> usize {
        let new_index = self.nodes.len();
        self.nodes.push(Node::new());
        new_index
    }
}

fn main() {
    let input_data = load_input();
    let mut sc = Scanner::new(&input_data);

    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());

    solve(&mut sc, &mut out);
}

fn solve(sc: &mut Scanner, out: &mut impl Write) {
    let binding = sc.next::<String>();
    let target = binding.as_bytes();
    let target_len = target.len();

    let num_words: usize = sc.next();

    let mut trie = Trie::new();
    for _ in 0..num_words {
        trie.insert(&sc.next::<String>());
    }

    let mut dp = vec![0_i64; target_len + 1];
    dp[target_len] = 1;

    for i in (0..target_len).rev() {
        let mut curr_idx = 0;

        for j in i..target_len {
            let char_idx = (target[j] - b'a') as usize;

            if let Some(next_idx) = trie.nodes[curr_idx].children[char_idx] {
                curr_idx = next_idx;

                if trie.nodes[curr_idx].count > 0 {
                    let ways = (trie.nodes[curr_idx].count * dp[j + 1]) % MOD;
                    dp[i] = (dp[i] + ways) % MOD;
                }
            } else {
                break;
            }
        }
    }

    // writeln!(out, "{:?}", dp).ok();
    writeln!(out, "{}", dp[0]).ok();
}

fn load_input() -> String {
    let mut input_data = String::new();
    #[cfg(debug_assertions)]
    {
        use std::fs::File;
        if let Ok(mut f) = File::open("data/input.txt") {
            f.read_to_string(&mut input_data).unwrap();
            return input_data;
        }
    }
    io::stdin().read_to_string(&mut input_data).unwrap();
    input_data
}
