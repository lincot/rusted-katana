//! <https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn neutralise(s1: &str, s2: &str) -> String {
    assert!(s1.is_ascii() && s1.len() == s2.len());
    let res = s1
        .bytes()
        .zip(s2.bytes())
        .map(|(a, b)| if a == b { a } else { b'0' })
        .collect();
    unsafe { String::from_utf8_unchecked(res) }
}
