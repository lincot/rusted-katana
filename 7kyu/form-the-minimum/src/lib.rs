//! <https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use vqsort::VqSort;

pub fn min_value(mut digits: Vec<i32>) -> i32 {
    VqSort::sort_ascending(&mut digits);
    digits.dedup();
    digits.into_iter().fold(0, |acc, d| 10 * acc + d)
}
