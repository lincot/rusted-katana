//! <https://www.codewars.com/kata/52b5247074ea613a09000164/train/rust>

#![no_std]

pub const fn cooking_time(eggs: u32) -> u32 {
    (eggs + 7) / 8 * 5
}
