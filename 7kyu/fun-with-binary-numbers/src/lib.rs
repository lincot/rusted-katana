//! <https://www.codewars.com/kata/5ce04eadd103f4001edd8986/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn solution(n: u8, b: u32) -> Vec<u32> {
    let len = if b == 0 || b >= 1 << n {
        0
    } else {
        1 << (n - 1)
    };

    let right_mask = b.wrapping_sub(1);
    let left_mask = !right_mask;

    let mut res = Vec::with_capacity(len as _);
    unsafe { res.set_len(len as _) };
    let mut res_ptr = res.as_mut_ptr();
    for i in 0..len {
        unsafe {
            *res_ptr = ((i & left_mask) << 1) | b | (i & right_mask);
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
