fn main() {
    let input = adapted_input();
    let mut sc = Scanner::new(&input);

    let t: usize = sc.next();
    for _ in 0..t { 
        let tc = TestCase::new(&mut sc);
        solution::solve(tc);
    }
}

mod solution {
    use super::*;
    const DX: [i32; 4] = [1, -1, 0, 0];
    const DY: [i32; 4] = [0, 0, 1, -1];

    fn check_range(x: i32, y: i32, n: usize, m: usize) -> bool {
        if x < 0 || y < 0 { return false; }
        if x as usize > n || y as usize > m { return false; }
        true
    }

    pub fn bfs(start_x: usize, start_y: usize, mat: &[Vec<usize>], visited: &mut [Vec<bool>]) -> usize {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_x, start_y));
        visited[start_x][start_y] = true;
        let mut ans = mat[start_x][start_y];

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < mat.len() && ny < mat[0].len() && !visited[nx][ny] && mat[nx][ny] != 0 {
                    visited[nx][ny] = true;
                    ans += mat[nx][ny];
                    queue.push_back((nx, ny));
                }
            }
        }
        ans
    }

    pub fn solve(tc: TestCase) {
        let TestCase { n, m, mat } = tc;
        let mut visited: Vec<Vec<_>> = mat.iter().map(|row| row.iter().map(|&x| x == 0 ).collect()).collect();
        
        let mut ans = 0;
        for i in 1..=n {
            for j in 1..=m {
                if visited[i][j] { continue; }
                ans = ans.max(bfs(i, j, &mat, &mut visited));
            }
        }
        println!("{}", ans);
    }
}

pub struct TestCase {
    n: usize,
    m: usize,
    mat: Vec<Vec<usize>>,
}

impl TestCase {
    pub fn new(sc: &mut Scanner) -> Self {
        let n = sc.next();
        let m = sc.next();
        let mut mat = vec![vec![0_usize; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                mat[i][j] = sc.next();
            }
        }
        Self { n, m, mat }
    }
}











use std::io::{self, Read};
use std::str::FromStr;

fn adapted_input() -> String {
    match cfg!(debug_assertions) {
        true => std::fs::read_to_string("input.txt").unwrap(),
        false => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            input
        }
    }
}

pub struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Scanner { iter: input.split_whitespace() }
    }
    fn next<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}