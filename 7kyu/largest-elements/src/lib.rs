//! <https://www.codewars.com/kata/53d32bea2f2a21f666000256/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::cmp::Reverse;

pub fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
    assert!(xs.len() >= n);
    let mut xs = xs.to_vec();
    if n != 0 {
        xs.select_nth_unstable_by_key(n - 1, |&x| Reverse(x));
    }
    xs.truncate(n);
    xs.sort_unstable();
    xs
}
