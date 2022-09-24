//! <https://www.codewars.com/kata/523f5d21c841566fde000009/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn array_diff<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));
    a
}
