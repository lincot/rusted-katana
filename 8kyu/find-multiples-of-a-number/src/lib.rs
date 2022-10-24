//! <https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let cap = (limit / n) as _;
    let mut res = Vec::with_capacity(cap);
    let mut i = n;
    for _ in 0..cap {
        unsafe {
            res.push_unchecked(i);
        }
        i += n;
    }
    res
}
