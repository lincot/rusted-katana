//! <https://www.codewars.com/kata/55dc4520094bbaf50e0000cb/train/rust>

#![no_std]

pub fn am_i_wilson(n: u32) -> bool {
    [5, 13, 563].contains(&n)
}
