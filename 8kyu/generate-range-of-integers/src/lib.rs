//! <https://www.codewars.com/kata/55eca815d0d20962e1000106/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn generate_range(mut min: usize, max: usize, step: usize) -> Vec<usize> {
    let cap = if min <= max {
        (max - min) / step + 1
    } else {
        0
    };
    let mut res = Vec::with_capacity(cap);
    for _ in 0..cap {
        unsafe { res.push_unchecked(min) };
        min += step;
    }
    res
}
