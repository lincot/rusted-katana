//! <https://www.codewars.com/kata/5963c18ecb97be020b0000a2/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked::PushStrUnchecked;

pub fn derive(coefficient: u32, exponent: u32) -> String {
    let mut res = String::with_capacity(2 * u32::MAX_LEN_BASE10 + 2);
    unsafe {
        res.write_num_unchecked(coefficient * exponent, 10, false, false);
        res.push_str_unchecked("x^");
        res.write_num_unchecked(exponent - 1, 10, false, false);
    }
    res
}
