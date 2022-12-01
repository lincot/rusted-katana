//! <https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a/train/rust>

#![no_std]
#![feature(array_windows)]

pub fn stones_to_remove(stones: &str) -> usize {
    stones
        .as_bytes()
        .array_windows()
        .filter(|&[a, b]| a == b)
        .count()
}
