//! <https://www.codewars.com/kata/58fa273ca6d84c158e000052/train/rust>

#![no_std]

#[allow(non_upper_case_globals)]
pub const digits: fn(u64) -> usize = prelude::CountDigits::count_digits;
