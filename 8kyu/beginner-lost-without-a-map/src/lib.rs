//! <https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn maps(values: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(values.len());
    unsafe { res.set_len(values.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in values {
        unsafe {
            *res_ptr = 2 * x;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
