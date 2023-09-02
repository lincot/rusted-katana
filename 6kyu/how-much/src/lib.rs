//! <https://www.codewars.com/kata/55b4d87a3766d9873a0000d4/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::mem::{swap, transmute, MaybeUninit};
use digital::WriteNumUnchecked;
use prelude::*;

pub fn how_much(mut m: i32, mut n: i32) -> Vec<(String, String, String)> {
    if m > n {
        swap(&mut m, &mut n);
    }
    let start = (m + 25) / 63;
    let end = (n + 26) / 63;
    let len = (end - start) as _;
    let mut res = Vec::with_capacity(len);
    unsafe {
        res.set_len(len);
        for (r, k) in res.iter_mut().zip(start..) {
            let mut res1 = String::with_capacity(3 + 11);
            res1.push_str_unchecked("M: ");
            res1.write_num_unchecked(k * 63 + 37, 10, false, false);
            let mut res2 = String::with_capacity(3 + 11);
            res2.push_str_unchecked("B: ");
            res2.write_num_unchecked(k * 9 + 5, 10, false, false);
            let mut res3 = String::with_capacity(3 + 11);
            res3.push_str_unchecked("C: ");
            res3.write_num_unchecked(k * 7 + 4, 10, false, false);
            *r = MaybeUninit::new((res1, res2, res3));
        }
        transmute(res)
    }
}
