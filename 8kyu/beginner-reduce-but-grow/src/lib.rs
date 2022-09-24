//! <https://www.codewars.com/kata/57f780909f7e8e3183000078/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn grow(nums: Vec<i32>) -> i32 {
    nums.into_iter().product()
}
