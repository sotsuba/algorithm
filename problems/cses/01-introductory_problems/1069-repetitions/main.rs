use std::io::{self, Read};
use std::cmp;

struct RepetitionState {
    last_char: Option<char>,
    current_streak: usize,
    max_streak: usize,
}

impl RepetitionState {
    pub fn new() -> Self {
        Self {
            last_char: None,
            current_streak: 0,
            max_streak: 0,
        }
    }

    pub fn consume(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            let c = byte as char;

            if Some(c) == self.last_char {
                self.current_streak += 1;
            }
            else {
                self.current_streak = 1;
                self.last_char = Some(c);
            }
            self.max_streak = cmp::max(self.max_streak, self.current_streak);
        }
    }
}

fn main() -> io::Result<()> {
    let mut state = RepetitionState::new();
    let mut buffer = [0u8; 3636];
    let mut stdin = io::stdin().lock();

    loop {
        let bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 { break; }

        state.consume(&buffer[..bytes_read]);
    }

    println!("{}", state.max_streak);
    Ok(())
}

// https://cses.fi/problemset/task/1069/
