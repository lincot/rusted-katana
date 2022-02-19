//! <https://www.codewars.com/kata/58fa273ca6d84c158e000052/train/rust>

#![feature(int_log)]

pub const fn digits(n: u64) -> usize {
    (n.log10() + 1) as _
}
