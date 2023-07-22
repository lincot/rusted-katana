//! <https://www.codewars.com/kata/62665d43e67fbaf7b37212d2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn bell(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as _);
    unsafe { res.set_len(n as _) };
    for (i, r) in (0..).zip(res.iter_mut()) {
        *r = (n - i) * (i + 1);
    }
    res
}
