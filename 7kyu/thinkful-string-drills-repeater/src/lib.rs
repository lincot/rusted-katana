//! <https://www.codewars.com/kata/585a1a227cb58d8d740001c3/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn repeater(string: &str, n: u32) -> String {
    string.repeat(n as _)
}
