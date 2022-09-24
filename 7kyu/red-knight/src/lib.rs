//! <https://www.codewars.com/kata/5fc4349ddb878a0017838d0f/train/rust>

#![no_std]

pub const fn red_knight(n: u64, p: u64) -> (&'static str, u64) {
    (if n == p % 2 { "White" } else { "Black" }, 2 * p)
}
