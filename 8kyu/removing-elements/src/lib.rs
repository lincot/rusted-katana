//! <https://www.codewars.com/kata/5769b3802ae6f8e4890009d2/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let len = (arr.len() + 1) / 2;
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    let mut res_ptr = res.as_mut_ptr();
    let mut i = 0;
    for _ in 0..len {
        unsafe {
            *res_ptr = *arr.get_unchecked(i);
            res_ptr = res_ptr.add(1);
        }

        i += 2;
    }
    res
}
