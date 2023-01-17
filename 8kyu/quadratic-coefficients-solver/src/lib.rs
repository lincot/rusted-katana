//! <https://www.codewars.com/kata/5d59576768ba810001f1f8d6/train/rust>

#![no_std]

pub const fn quadratic(x1: i32, x2: i32) -> (i32, i32, i32) {
    (1, -(x1 + x2), x1 * x2)
}
