//! <https://www.codewars.com/kata/5959ec605595565f5c00002b/train/rust>

#![no_std]

pub const fn reverse_bits(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n.reverse_bits() >> n.leading_zeros()
    }
}
