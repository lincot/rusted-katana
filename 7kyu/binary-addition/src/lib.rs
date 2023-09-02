//! <https://www.codewars.com/kata/551f37452ff852b7bd000139/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;

pub fn add_binary(a: u64, b: u64) -> String {
    let mut res = String::with_capacity(64);
    unsafe { res.write_num_unchecked(a + b, 2, false, false) };
    res
}
