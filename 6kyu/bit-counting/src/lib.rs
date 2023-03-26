//! <https://www.codewars.com/kata/526571aae218b8ee490006f4/train/rust>

#![no_std]

#[allow(non_upper_case_globals)]
pub const count_bits: fn(i64) -> u32 = i64::count_ones;
