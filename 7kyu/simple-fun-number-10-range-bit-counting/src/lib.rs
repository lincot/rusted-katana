//! <https://www.codewars.com/kata/58845748bd5733f1b300001f/train/rust>

#![no_std]

pub fn range_bit_count(a: u32, b: u32) -> u32 {
    range_bit_count_to(b) - if a == 0 { 0 } else { range_bit_count_to(a - 1) }
}

fn range_bit_count_to(n: u32) -> u32 {
    (0..7)
        .map(|i| {
            let b = 1 << i;
            if n & b == 0 {
                0
            } else if i == 0 {
                1
            } else {
                1 + (n & (b - 1)) + (i << (i - 1))
            }
        })
        .sum()
}
