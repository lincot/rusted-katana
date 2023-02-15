//! <https://www.codewars.com/kata/57ea70aa5500adfe8a000110/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn fold_array(arr: &[i32], runs: usize) -> Vec<i32> {
    let mut res = arr.to_vec();
    let mut len = arr.len();
    for _ in 0..runs {
        for i in 0..len / 2 {
            unsafe { *res.get_unchecked_mut(i) += *res.get_unchecked(len - i - 1) };
        }
        len -= len / 2;
    }
    unsafe { res.set_len(len) };
    res
}
