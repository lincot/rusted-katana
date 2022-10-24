//! <https://www.codewars.com/kata/563cf89eb4747c5fb100001b/train/rust>

#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};
use prelude::*;

pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.len() < 2 {
        return vec![];
    }

    let mut min_pos = 0;
    for (i, x) in (1..).zip(&numbers[1..]) {
        if x < unsafe { numbers.get_unchecked(min_pos) } {
            min_pos = i;
        }
    }

    let mut res = Vec::with_capacity(numbers.len());
    unsafe {
        res.extend_from_slice_unchecked(numbers.get_unchecked(..min_pos));
        res.extend_from_slice_unchecked(numbers.get_unchecked(min_pos + 1..));
    }
    res
}
