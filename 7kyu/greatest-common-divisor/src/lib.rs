//! <https://www.codewars.com/kata/5500d54c2ebe0a8e8a0003fd/train/rust>

#![no_std]

#[allow(non_upper_case_globals)]
pub const gcd: fn(u32, u32) -> u32 = num_integer::gcd;
