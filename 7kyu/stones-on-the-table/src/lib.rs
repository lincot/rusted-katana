//! <https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a/train/rust>

pub fn stones_to_remove(stones: &str) -> usize {
    stones
        .as_bytes()
        .windows(2)
        .filter(|&pair| pair[0] == pair[1])
        .count()
}
