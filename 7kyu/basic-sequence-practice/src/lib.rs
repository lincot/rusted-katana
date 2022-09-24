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
    let mut res_ptr = res.as_mut_ptr();
    if n > 0 {
        for x in 0..len {
            let x = x as i32;
            unsafe {
                *res_ptr = (x * x + x) / 2;
                res_ptr = res_ptr.add(1);
            }
        }
    } else {
        for x in 0..len {
            let x = x as i32;
            unsafe {
                *res_ptr = -x * (x + 1) / 2;
                res_ptr = res_ptr.add(1);
            }
        }
    }
    res
}
