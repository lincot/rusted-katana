//! <https://www.codewars.com/kata/5545f109004975ea66000086/train/rust>

#![no_std]

pub const fn is_divisible(n: i32, x: i32, y: i32) -> bool {
    n % x == 0 && n % y == 0
}
