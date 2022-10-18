//! <https://www.codewars.com/kata/617dcb2f242452004a77c653/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn merge<'a>(xs: &'a Vec<usize>, ys: &'a Vec<usize>) -> Vec<&'a usize> {
    let mut res = Vec::with_capacity(xs.len() + ys.len());
    unsafe { res.set_len(xs.len() + ys.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for x in xs {
        unsafe {
            *res_ptr = x;
            res_ptr = res_ptr.add(1);
        }
    }
    for y in ys {
        unsafe {
            *res_ptr = y;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
