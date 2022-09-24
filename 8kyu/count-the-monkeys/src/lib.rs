//! <https://www.codewars.com/kata/56f69d9f9400f508fb000ba7/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn monkey_count(n: i32) -> Vec<i32> {
    let n = n.max(0);
    let mut res = Vec::with_capacity(n as _);
    unsafe { res.set_len(n as _) };
    let mut res_ptr = res.as_mut_ptr();
    let mut i = 0;
    while i != n {
        i += 1;
        unsafe {
            *res_ptr = i;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
