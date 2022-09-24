//! <https://www.codewars.com/kata/571d42206414b103dc0006a1/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn arr(n: usize) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as u32 as _);
    unsafe { res.set_len(n as u32 as _) };
    let mut res_ptr = res.as_mut_ptr();
    for i in 0..n as u32 {
        unsafe {
            *res_ptr = i;
            res_ptr = res_ptr.add(1);
        };
    }
    res
}
