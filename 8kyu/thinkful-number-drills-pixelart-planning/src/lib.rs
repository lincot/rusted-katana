//! <https://www.codewars.com/kata/58630e2ae88af44d2b0000ea/train/rust>

#![no_std]

pub const fn is_divisible(wall: i32, pixel: i32) -> bool {
    wall % pixel == 0
}
