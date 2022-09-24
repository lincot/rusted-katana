//! <https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use my_prelude::prelude::*;

pub fn even_numbers(array: &[i32], number: usize) -> Vec<i32> {
    let mut res = Vec::with_capacity(number);
    for &x in array.iter().rev() {
        if res.len() == number {
            break;
        }
        if x % 2 == 0 {
            unsafe { res.push_unchecked(x) };
        }
    }
    res.reverse();
    res
}
