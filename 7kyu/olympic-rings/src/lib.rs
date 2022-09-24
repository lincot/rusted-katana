//! <https://www.codewars.com/kata/57d06663eca260fe630001cc/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn olympic_ring(s: &str) -> String {
    let rings: usize = s
        .bytes()
        .map(|b| {
            if b"eaodpgbqARODPQ".contains(&b) {
                1
            } else if b == b'B' {
                2
            } else {
                0
            }
        })
        .sum();

    match rings / 2 {
        0..=1 => "Not even a medal!",
        2 => "Bronze!",
        3 => "Silver!",
        _ => "Gold!",
    }
    .into()
}
