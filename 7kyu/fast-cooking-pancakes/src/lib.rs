//! <https://www.codewars.com/kata/58552bdb68b034a1a80001fb/train/rust>

pub fn cook_pancakes(n: u32, m: u32) -> u32 {
    ((2 * n + m - 1) / m).max(2)
}
