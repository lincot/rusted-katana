//! <https://www.codewars.com/kata/5265326f5fda8eb1160004c8/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::NumToString;

pub fn number_to_string(i: i32) -> String {
    i.to_string(false, false)
}
