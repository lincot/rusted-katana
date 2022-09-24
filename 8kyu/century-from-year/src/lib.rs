//! <https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/train/rust>

#![no_std]

pub const fn century(year: u32) -> u32 {
    1 + (year - 1) / 100
}
