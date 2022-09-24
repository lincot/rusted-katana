//! <https://www.codewars.com/kata/5174a4c0f2769dd8b1000003/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn sort_numbers(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    arr.sort_unstable();
    arr
}
