//! <https://www.codewars.com/kata/59590976838112bfea0000fa/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    let mut res = Vec::with_capacity(n);
    unsafe { res.set_len(n) };
    let mut res_ptr = res.as_mut_ptr();
    for i in 0..n {
        unsafe {
            *res_ptr = values.iter().skip(i).step_by(n).sum::<u32>();
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
