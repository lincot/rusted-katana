//! <https://www.codewars.com/kata/57f75cc397d62fc93d000059/train/rust>

#![no_std]

pub fn calc(s: &str) -> u32 {
    s.bytes()
        .map(|b| 6 * (b % 100 / 10 == 7) as u32 + 6 * (b % 10 == 7) as u32)
        .sum()
}
