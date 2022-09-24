//! <https://www.codewars.com/kata/5aba780a6a176b029800041c/train/rust>

#![no_std]

pub const fn max_multiple(divisor: u32, bound: u32) -> u32 {
    bound / divisor * divisor
}
