//! <https://www.codewars.com/kata/56b97b776ffcea598a0006f2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(lst.len());
    unsafe { res.set_len(lst.len()) };
    if lst.is_empty() {
        return res;
    }
    let mut max = lst[0];
    for (r, &x) in res.iter_mut().zip(&lst[1..]) {
        if x > max {
            *r = max;
            max = x;
        } else {
            *r = x;
        }
    }
    *res.last_mut().unwrap() = max;
    res
}
