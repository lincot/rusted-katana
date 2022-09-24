//! <https://www.codewars.com/kata/57eae65a4321032ce000002d/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};

pub fn fake_bin(s: &str) -> String {
    let mut s = s.to_string();
    for b in unsafe { s.as_bytes_mut() } {
        *b = if *b < b'5' { b'0' } else { b'1' };
    }
    s
}
