//! <https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn digitize(n: u64) -> Vec<u8> {
    let mut res = Vec::with_capacity(20);
    unsafe { res.write_num_unchecked(n, true, true) };
    res
}
