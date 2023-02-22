//! <https://www.codewars.com/kata/5cd4aec6abc7260028dcd942/train/rust>

#![no_std]

pub const fn shortest_steps_to_num(n: u16) -> u16 {
    (n.count_ones() + 14 - n.leading_zeros()) as _
}
