//! <https://www.codewars.com/kata/56dec885c54a926dcd001095/train/rust>

#[allow(non_upper_case_globals)]
pub const opposite: fn(i32) -> i32 = std::ops::Neg::neg;
