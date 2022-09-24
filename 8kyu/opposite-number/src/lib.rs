//! <https://www.codewars.com/kata/56dec885c54a926dcd001095/train/rust>

#![no_std]

#[allow(non_upper_case_globals)]
pub const opposite: fn(i32) -> i32 = core::ops::Neg::neg;
