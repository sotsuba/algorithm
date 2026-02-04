const ALPHABET_SIZE: usize = 26;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let idx = letters.partition_point(|&x| x <= target);
        if idx == letters.len() {
            letters[0]
        } else {
            letters[idx]
        }
    }
}
