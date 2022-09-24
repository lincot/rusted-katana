//! <https://www.codewars.com/kata/62a611067274990047f431a8/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use my_prelude::prelude::*;

pub fn alternate<'a>(n: usize, first_value: &'a str, second_value: &'a str) -> Vec<&'a str> {
    let mut res = Vec::with_capacity(n);
    for _ in 0..n / 2 {
        unsafe {
            res.push_unchecked(first_value);
            res.push_unchecked(second_value);
        }
    }
    if n % 2 == 1 {
        unsafe {
            res.push_unchecked(first_value);
        }
    }
    res
}
