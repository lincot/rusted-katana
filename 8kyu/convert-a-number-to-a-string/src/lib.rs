//! <https://www.codewars.com/kata/5265326f5fda8eb1160004c8/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;

pub fn number_to_string(i: i32) -> String {
    let mut res = String::with_capacity(11);
    unsafe { res.write_num_unchecked(i, 10, false, false) };
    res
}
