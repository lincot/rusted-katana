//! <https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn row_weights(array: Vec<u32>) -> (u32, u32) {
    array
        .into_iter()
        .enumerate()
        .fold((0, 0), |(t1, t2), (i, w)| {
            if i % 2 == 0 {
                (t1 + w, t2)
            } else {
                (t1, t2 + w)
            }
        })
}
