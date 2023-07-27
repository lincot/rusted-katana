//! <https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;
use vqsort::VqSort;

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = Vec::with_capacity(arr.len());
    for &x in arr {
        if x % 2 == 1 {
            unsafe { odds.push_unchecked(x) };
        }
    }
    VqSort::sort_ascending(&mut odds);

    let mut res = Vec::with_capacity(arr.len());
    unsafe { res.set_len(arr.len()) };
    let mut odd_i = 0;
    for (r, &x) in res.iter_mut().zip(arr) {
        if x % 2 == 1 {
            *r = unsafe { *odds.get_unchecked(odd_i) };
            odd_i += 1;
        } else {
            *r = x;
        }
    }
    res
}
