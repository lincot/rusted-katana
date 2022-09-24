//! <https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn high_and_low(numbers: &str) -> String {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for n in numbers.as_bytes().split(|&b| b == b' ').map(|s| {
        unsafe { core::str::from_utf8_unchecked(s) }
            .parse()
            .unwrap()
    }) {
        min = min.min(n);
        max = max.max(n);
    }

    let mut res = String::with_capacity(11 + 1 + 11);
    unsafe {
        res.write_num_unchecked(max);
        res.push_unchecked(' ');
        res.write_num_unchecked(min);
    }
    res
}
