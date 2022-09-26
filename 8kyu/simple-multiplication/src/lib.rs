//! <https://www.codewars.com/kata/583710ccaa6717322c000105/train/rust>

#![no_std]

pub const fn simple_multiplication(number: u8) -> u8 {
    number * (8 + number % 2)
}
