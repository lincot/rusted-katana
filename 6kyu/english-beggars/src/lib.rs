//! <https://www.codewars.com/kata/59590976838112bfea0000fa/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    let mut res = Vec::with_capacity(n);
    unsafe { res.set_len(n) };
    for (i, r) in res.iter_mut().enumerate() {
        *r = values.iter().skip(i).step_by(n).sum::<u32>();
    }
    res
}
