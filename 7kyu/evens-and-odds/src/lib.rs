//! <https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;

pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        let mut res = String::with_capacity(64);
        unsafe { res.write_num_unchecked(n, 2, false, false) };
        res
    } else {
        let mut res = String::with_capacity(64 / 4);
        unsafe { res.write_num_unchecked(n, 16, false, false) };
        res
    }
}
