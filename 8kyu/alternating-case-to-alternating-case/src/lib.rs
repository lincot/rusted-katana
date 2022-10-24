//! <https://www.codewars.com/kata/56efc695740d30f963000557/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn to_alternating_case(s: &str) -> String {
    let mut res = String::with_capacity(3 * s.len());

    for c in s.chars() {
        if c.is_lowercase() {
            unsafe { res.extend_unchecked(c.to_uppercase()) };
        } else {
            unsafe { res.extend_unchecked(c.to_lowercase()) };
        }
    }

    res
}
