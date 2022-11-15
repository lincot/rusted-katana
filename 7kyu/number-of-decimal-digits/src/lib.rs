//! <https://www.codewars.com/kata/58fa273ca6d84c158e000052/train/rust>

#![no_std]

pub const fn digits(n: u64) -> usize {
    (n.ilog10() + 1) as _
}
