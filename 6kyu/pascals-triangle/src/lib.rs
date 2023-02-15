//! <https://www.codewars.com/kata/5226eb40316b56c8d500030f/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn pascals_triangle(n: usize) -> Vec<usize> {
    let cap = n * (n + 1) / 2;
    let mut res = Vec::with_capacity(cap);
    let mut ptr = res.as_mut_ptr();
    unsafe {
        res.set_len(cap);
        *ptr = 1;
        ptr = ptr.add(1);
        for i in 1..n {
            *ptr = 1;
            ptr = ptr.add(1);
            for _ in 1..i {
                *ptr = *ptr.sub(i + 1) + *ptr.sub(i);
                ptr = ptr.add(1);
            }
            *ptr = 1;
            ptr = ptr.add(1);
        }
    }
    res
}
