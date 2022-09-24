//! <https://www.codewars.com/kata/59706036f6e5d1e22d000016/train/rust>

#![no_std]

pub fn words_to_marks(s: &str) -> u32 {
    s.bytes().map(|b| (b - (b'a' - 1)) as u32).sum()
}
