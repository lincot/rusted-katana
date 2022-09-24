//! <https://www.codewars.com/kata/5ce9c1000bab0b001134f5af/train/rust>

#![no_std]

pub const fn quarter_of(month: u8) -> u8 {
    (month + 2) / 3
}
