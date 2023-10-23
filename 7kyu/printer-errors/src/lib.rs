//! <https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::{MaxLenBase10, WriteNumUnchecked};
use prelude::*;

pub fn printer_error(s: &str) -> String {
    let malformed = s.bytes().filter(|&b| b > b'm').count();
    let all = s.len();

    let mut res = String::with_capacity(2 * usize::MAX_LEN_BASE10 + 1);
    unsafe {
        res.write_num_unchecked(malformed, 10, false, false);
        res.push_unchecked('/');
        res.write_num_unchecked(all, 10, false, false);
    }
    res
}
