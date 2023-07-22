//! <https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn invert(values: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(values.len());
    unsafe { res.set_len(values.len()) };
    for (r, &x) in res.iter_mut().zip(values) {
        *r = -x;
    }
    res
}
