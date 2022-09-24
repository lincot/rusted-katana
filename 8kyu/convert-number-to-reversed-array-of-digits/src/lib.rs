//! <https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust>

#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};
use my_prelude::prelude::*;

pub fn digitize(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut res = Vec::with_capacity(20);

    // TODO: make a better conversion
    while n != 0 {
        unsafe { res.push_unchecked((n % 10) as _) };
        n /= 10;
    }

    res
}
