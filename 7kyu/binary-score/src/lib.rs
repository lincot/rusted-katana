//! <https://www.codewars.com/kata/56cafdabc8cfcc3ad4000a2b/train/rust>

#![no_std]

pub const fn score(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        u32::MAX >> n.leading_zeros()
    }
}
