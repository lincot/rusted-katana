//! <https://www.codewars.com/kata/599c20626bd8795ce900001d/train/rust>

pub fn summy(strng: &str) -> i32 {
    strng
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}
