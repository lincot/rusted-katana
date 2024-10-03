//! <https://www.codewars.com/kata/56b1f01c247c01db92000076/train/rust>

use unchecked_std::prelude::*;

pub fn double_char(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(double_byte(s.as_bytes())) };
    }

    let mut res = String::with_capacity(2 * s.len());
    for ch in s.chars() {
        unsafe {
            res.push_unchecked(ch);
            res.push_unchecked(ch);
        }
    }
    res
}

fn double_byte(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(2 * s.len());
    for &b in s {
        unsafe {
            res.push_unchecked(b);
            res.push_unchecked(b);
        }
    }
    res
}
