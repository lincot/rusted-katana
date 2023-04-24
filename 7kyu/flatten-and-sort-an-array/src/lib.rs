//! <https://www.codewars.com/kata/57ee99a16c8df7b02d00045f/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let len = arr.iter().map(Vec::len).sum();
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    let mut res_ptr = res.as_mut_ptr();
    for x in arr {
        for &x in x {
            unsafe {
                *res_ptr = x;
                res_ptr = res_ptr.add(1);
            }
        }
    }
    res.sort_unstable();
    res
}
