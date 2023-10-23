//! <https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use unchecked::PushUnchecked;

pub fn solution(s: &str) -> String {
    let mut res = String::with_capacity(2 * s.len());
    for c in s.chars() {
        if c.is_uppercase() {
            unsafe { res.push_unchecked(' ') };
        }
        unsafe { res.push_unchecked(c) };
    }
    res
}
