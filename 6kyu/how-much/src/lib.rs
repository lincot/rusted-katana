//! <https://www.codewars.com/kata/55b4d87a3766d9873a0000d4/train/rust>

use core::mem::swap;
use my_prelude::prelude::*;

pub fn how_much(mut m: i32, mut n: i32) -> Vec<(String, String, String)> {
    if m > n {
        swap(&mut m, &mut n);
    }

    ((m + 25) / 63..(n + 26) / 63)
        .map(|k| {
            let mut res1 = String::with_capacity(3 + 11);
            unsafe {
                res1.push_str_unchecked("M: ");
                res1.write_num_unchecked(k * 63 + 37);
            }
            let mut res2 = String::with_capacity(3 + 11);
            unsafe {
                res2.push_str_unchecked("B: ");
                res2.write_num_unchecked(k * 9 + 5);
            }
            let mut res3 = String::with_capacity(3 + 11);
            unsafe {
                res3.push_str_unchecked("C: ");
                res3.write_num_unchecked(k * 7 + 4);
            }

            (res1, res2, res3)
        })
        .collect()
}
