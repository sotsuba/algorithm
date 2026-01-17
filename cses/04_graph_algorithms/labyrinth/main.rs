use std::collections::VecDeque;
use std::io::{self, Read};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum Move {
    Up = 0, 
    Down = 1, 
    Left = 2, 
    Right = 3, 
    Untouched = 4,
    Start = 5,
}

impl Move {
    const ALL: [Move; 4] = [Move::Up, Move::Down, Move::Left, Move::Right];

    fn name(&self) -> char {
        match self {
            Move::Up => 'U',
            Move::Down => 'D',
            Move::Left => 'L',
            Move::Right => 'R',
            _ => '_', 
        }
    }

    fn delta(&self, width: usize) -> isize {
        match self {
            Move::Up => -(width as isize),
            Move::Down => width as isize,
            Move::Left => -1,
            Move::Right => 1,
            _ => 0,
        }
    }

    fn try_advance(self, cur: usize, h: usize, w: usize) -> Option<usize> {
        let r = cur / w;
        let c = cur % w;
        match self {
            Move::Up if r > 0 => Some(cur - w),
            Move::Down if r < h - 1 => Some(cur + w),
            Move::Left if c > 0 => Some(cur - 1),
            Move::Right if c < w - 1 => Some(cur + 1),
            _ => None,
        }
    }

    fn try_recede(self, cur: usize, h: usize, w: usize) -> Option<usize> {
        let r = cur / w;
        let c = cur % w;
        match self {
            Move::Up if r < h - 1 => Some(cur + w),
            Move::Down if r > 0 => Some(cur - w),
            Move::Left if c < w - 1 => Some(cur + 1),
            Move::Right if c > 0 => Some(cur - 1),
            _ => None,
        }
    }
}

struct Labyrinth {
    h: usize, w: usize,
    grid: Vec<u8>,
    src: usize, des: usize,
}

impl Labyrinth {
    pub fn new(buffer: &str) -> Self {
        let mut tokens = buffer.split_whitespace();

        let h: usize = tokens.next().unwrap().parse().unwrap();
        let w: usize = tokens.next().unwrap().parse().unwrap();
        let grid: Vec<u8> = tokens.flat_map(|s| s.as_bytes().iter().copied()).collect();
        let mut src: usize = 0;
        let mut des: usize = 0;
        
        for (i, &byte) in grid.iter().enumerate() {
            if byte == b'A' {
                src = i;
            }
            if byte == b'B' {
                des = i;
            }
        }

        Self { h, w, grid, src, des }
    }

    pub fn bfs(&self) -> Option<String> {
        let size: usize = self.h * self.w;
        let mut parent: Vec<Move> = vec![Move::Untouched; size];
        let mut queue = VecDeque::with_capacity(size);
        
        parent[self.src] = Move::Start;
        queue.push_back(self.src);
        
        let mut found = false;
        
        while let Some(cur) = queue.pop_front() {
            if cur == self.des {
                return Some(self.reconstruct_path(parent));
            }

            for &mv in &Move::ALL {
                if let Some(nxt) = mv.try_advance(cur, self.h, self.w) {
                    if self.grid[nxt] != b'#' && parent[nxt] == Move::Untouched {
                        parent[nxt] = mv;
                        queue.push_back(nxt);
                    }
                }
            }
        }
        None 
    } 

    fn reconstruct_path(&self, parent: Vec<Move>) -> String {
        let mut path = Vec::new();
        let mut cur  = self.des;

        while cur != self.src {
            let mv = parent[cur];
            path.push(mv.name());
            cur = mv.try_recede(cur, self.h, self.w)
                .expect("logic error");
        }
        path.into_iter().rev().collect()
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let labyrinth = Labyrinth::new(&buffer);
    match labyrinth.bfs() {
        Some(path) => {
            println!("YES");
            println!("{}", path.len());
            println!("{}", path);
        },
        None => println!("NO"),
    }
}