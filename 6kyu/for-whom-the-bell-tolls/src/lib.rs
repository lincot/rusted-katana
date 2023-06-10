//! <https://www.codewars.com/kata/62665d43e67fbaf7b37212d2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn bell(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as _);
    unsafe { res.set_len(n as _) };
    let mut ptr = res.as_mut_ptr();
    for i in 0..n {
        unsafe {
            *ptr = (n - i) * (i + 1);
            ptr = ptr.add(1);
        }
    }
    res
}
