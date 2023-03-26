//! <https://www.codewars.com/kata/64060d8ab2dd990058b7f8ee/train/rust>

#![no_std]

pub const fn queens(n: i64) -> i64 {
    if n <= 0 {
        0
    } else {
        n * (n - 3) + 2
    }
}
