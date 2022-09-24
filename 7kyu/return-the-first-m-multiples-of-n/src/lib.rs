//! <https://www.codewars.com/kata/593c9175933500f33400003e/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn multiples(m: i32, n: f64) -> Vec<f64> {
    let m = m.max(0);
    let mut res = Vec::with_capacity(m as usize);
    unsafe { res.set_len(m as usize) };
    let mut res_ptr = res.as_mut_ptr();
    let mut i = 0;
    while i < m {
        i += 1;
        unsafe {
            *res_ptr = i as f64 * n;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
