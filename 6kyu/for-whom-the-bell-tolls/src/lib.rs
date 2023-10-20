//! <https://www.codewars.com/kata/62665d43e67fbaf7b37212d2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn bell(n: u32) -> Vec<u32> {
    (0..n).map(|i| (n - i) * (i + 1)).collect()
}
