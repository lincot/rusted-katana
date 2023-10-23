//! <https://www.codewars.com/kata/51c89385ee245d7ddf000001/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::{MaxLenBase10, WriteNumUnchecked};
use prelude::*;

pub fn solution(n: u32) -> String {
    let mut res = String::with_capacity("Value is ".len() + u32::MAX_LEN_BASE10);
    unsafe {
        res.push_str_unchecked("Value is ");
        if n < 10 {
            res.push_str_unchecked("0000");
        } else if n < 100 {
            res.push_str_unchecked("000");
        } else if n < 1000 {
            res.push_str_unchecked("00");
        } else if n < 10000 {
            res.push_unchecked('0');
        }
        res.write_num_unchecked(n, 10, false, false);
    }
    res
}
