//! <https://www.codewars.com/kata/50654ddff44f800200000004/train/rust>

#![no_std]

#[allow(non_upper_case_globals)]
pub const multiply: fn(i32, i32) -> i32 = core::ops::Mul::mul;
