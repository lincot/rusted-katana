//! <https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn solve(strings: &[String]) -> Vec<usize> {
    let mut res = Vec::with_capacity(strings.len());
    unsafe { res.set_len(strings.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for string in strings {
        unsafe {
            *res_ptr = b"abcdefghijklmnopqrstuvwxyz"
                .iter()
                .zip(string.as_bytes())
                .filter(|&(&i, &b)| i == b || i - b'a' + b'A' == b)
                .count();
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
