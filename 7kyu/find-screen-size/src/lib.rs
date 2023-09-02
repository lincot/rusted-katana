//! <https://www.codewars.com/kata/5bbd279c8f8bbd5ee500000f/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;
use prelude::*;

pub fn find_screen_height(width: u64, ratio: &str) -> String {
    let (vertical, horizontal) = ratio.split_once(':').unwrap();

    let vertical: u64 = vertical.parse().unwrap();
    let horizontal: u64 = horizontal.parse().unwrap();

    let height = width * horizontal / vertical;

    let mut res = String::with_capacity(20 + 1 + 20);
    unsafe {
        res.write_num_unchecked(width, 10, false, false);
        res.push_unchecked('x');
        res.write_num_unchecked(height, 10, false, false);
    }
    res
}
