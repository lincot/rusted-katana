//! <https://www.codewars.com/kata/57ae18c6e298a7a6d5000c7a/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn replace_all<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<T> {
    let mut res = Vec::with_capacity(xs.len());
    unsafe { res.set_len(xs.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for &x in xs {
        unsafe {
            *res_ptr = if x == find { replace } else { x };
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
