//! <https://www.codewars.com/kata/5a905c2157c562994900009d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::hint::unreachable_unchecked;

pub fn product_array(arr: &[u64]) -> Vec<u64> {
    let prod: u64 = arr.iter().product();
    assert!(prod != 0);
    let mut res = Vec::with_capacity(arr.len());
    unsafe { res.set_len(arr.len()) };
    for (r, &x) in res.iter_mut().zip(arr) {
        if x == 0 {
            unsafe { unreachable_unchecked() };
        }
        *r = prod / x;
    }
    res
}
