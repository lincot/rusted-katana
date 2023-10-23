//! <https://www.codewars.com/kata/56b1f01c247c01db92000076/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use unchecked::PushUnchecked;

pub fn double_char(s: &str) -> String {
    let mut res = String::with_capacity(2 * s.len());
    for c in s.chars() {
        unsafe {
            res.push_unchecked(c);
            res.push_unchecked(c);
        }
    }
    res
}
