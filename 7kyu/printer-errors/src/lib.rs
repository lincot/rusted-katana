//! <https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn printer_error(s: &str) -> String {
    let malformed = s.bytes().filter(|&b| b > b'm').count();
    let all = s.len();

    let mut res = String::with_capacity(20 + 1 + 20);
    unsafe {
        res.write_num_unchecked(malformed);
        res.push_unchecked('/');
        res.write_num_unchecked(all);
    }
    res
}
