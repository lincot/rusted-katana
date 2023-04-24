//! <https://www.codewars.com/kata/54fb853b2c8785dd5e000957/train/rust>

#![no_std]

pub fn chain<F: Fn(i32) -> i32>(input: i32, functions: &[F]) -> i32 {
    functions.iter().fold(input, |acc, f| f(acc))
}
