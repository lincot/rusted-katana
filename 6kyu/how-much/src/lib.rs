//! <https://www.codewars.com/kata/55b4d87a3766d9873a0000d4/train/rust>

use core::mem::{swap, transmute, MaybeUninit};
use my_prelude::prelude::*;

pub fn how_much(mut m: i32, mut n: i32) -> Vec<(String, String, String)> {
    if m > n {
        swap(&mut m, &mut n);
    }
    let start = (m + 25) / 63;
    let end = (n + 26) / 63;
    let len = (end - start) as _;
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    let mut res_ptr = res.as_mut_ptr();
    for k in start..end {
        unsafe {
            let mut res1 = String::with_capacity(3 + 11);
            res1.push_str_unchecked("M: ");
            res1.write_num_unchecked(k * 63 + 37);
            let mut res2 = String::with_capacity(3 + 11);
            res2.push_str_unchecked("B: ");
            res2.write_num_unchecked(k * 9 + 5);
            let mut res3 = String::with_capacity(3 + 11);
            res3.push_str_unchecked("C: ");
            res3.write_num_unchecked(k * 7 + 4);

            *res_ptr = MaybeUninit::new((res1, res2, res3));
            res_ptr = res_ptr.add(1);
        }
    }
    unsafe { transmute(res) }
}
