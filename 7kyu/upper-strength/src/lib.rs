//! <https://www.codewars.com/kata/571640812ad763313600132b/train/rust>

#![no_std]

pub const fn alex_mistakes(number_of_katas: u32, time_limit: u32) -> u32 {
    u32::BITS - 1 - ((time_limit - 6 * number_of_katas) / 5 + 1).leading_zeros()
}
