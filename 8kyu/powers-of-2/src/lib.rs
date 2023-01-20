//! <https://www.codewars.com/kata/57a083a57cb1f31db7000028/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn powers_of_two(n: u8) -> Vec<u128> {
    let n = n as usize + 1;
    let mut res = Vec::with_capacity(n);
    unsafe { res.set_len(n) };
    let mut res_ptr = res.as_mut_ptr();
    let mut x = 1;
    for _ in 0..n {
        unsafe {
            *res_ptr = x;
            res_ptr = res_ptr.add(1);
        }
        x <<= 1;
    }
    res
}
