//! <https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn sum_of_n(n: i32) -> Vec<i32> {
    let len = if n > 0 {
        n as usize + 1
    } else {
        (-(n as isize)) as usize + 1
    };
    let mut res = Vec::with_capacity(len);
    unsafe { res.set_len(len) };
    if n > 0 {
        for (i, r) in (0..).zip(res.iter_mut()) {
            *r = (i * i + i) / 2;
        }
    } else {
        for (i, r) in (0..).zip(res.iter_mut()) {
            *r = -i * (i + 1) / 2;
        }
    }
    res
}
