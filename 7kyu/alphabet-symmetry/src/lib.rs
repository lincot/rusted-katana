//! <https://www.codewars.com/kata/59d9ff9f7905dfeed50000b0/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn solve(strings: &[String]) -> Vec<usize> {
    let mut res = Vec::with_capacity(strings.len());
    unsafe { res.set_len(strings.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for string in strings {
        #[allow(clippy::range_plus_one)]
        unsafe {
            *res_ptr = (b'a'..b'z' + 1)
                .zip(b'A'..b'Z' + 1)
                .zip(string.as_bytes())
                .filter(|&(lu, b)| <[_; 2]>::from(lu).contains(b))
                .count();
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
