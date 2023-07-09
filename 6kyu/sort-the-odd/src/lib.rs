//! <https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = Vec::with_capacity(arr.len());
    for &x in arr {
        if x % 2 == 1 {
            unsafe { odds.push_unchecked(x) };
        }
    }
    if odds.len() < 160 {
        odds.sort_unstable();
    } else {
        radsort::sort(&mut odds);
    }

    let mut res = Vec::with_capacity(arr.len());
    unsafe { res.set_len(arr.len()) };
    let mut res_ptr = res.as_mut_ptr();
    let mut odd_i = 0;
    for &x in arr {
        if x % 2 == 1 {
            unsafe { *res_ptr = *odds.get_unchecked(odd_i) };
            odd_i += 1;
        } else {
            unsafe { *res_ptr = x };
        }
        res_ptr = unsafe { res_ptr.add(1) };
    }
    res
}
