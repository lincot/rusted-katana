//! <https://www.codewars.com/kata/5226eb40316b56c8d500030f/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use unchecked::PushUnchecked;

pub fn pascals_triangle(n: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(n * (n + 1) / 2);
    unsafe {
        res.push_unchecked(1);
        for i in 1..n {
            res.push_unchecked(1);
            for j in res.len() - i..res.len() - 1 {
                res.push_unchecked(res.get_unchecked(j - 1) + res.get_unchecked(j));
            }
            res.push_unchecked(1);
        }
    }
    res
}
