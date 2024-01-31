//! <https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust>

use core::ops::Neg;

pub fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(Neg::neg).collect()
}
