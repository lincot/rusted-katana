//! <https://www.codewars.com/kata/593c9175933500f33400003e/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn multiples(m: i32, n: f64) -> Vec<f64> {
    let m = m.max(0);
    let mut res = Vec::with_capacity(m as _);
    unsafe { res.set_len(m as _) };
    for (i, r) in (1..).zip(res.iter_mut()) {
        *r = i as f64 * n;
    }
    res
}
