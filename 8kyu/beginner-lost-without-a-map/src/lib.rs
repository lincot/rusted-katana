//! <https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn maps(values: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(values.len());
    for &x in values {
        unsafe { res.push_unchecked(x) };
    }
    unsafe { res.set_len(values.len()) };
    res
}
