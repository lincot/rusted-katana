//! <https://www.codewars.com/kata/590e03aef55cab099a0002e8/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn incrementer(nums: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(nums.len());
    unsafe { res.set_len(nums.len()) };
    for (r, (i, num)) in res.iter_mut().zip(nums.iter().enumerate()) {
        *r = (i as u32 + 1 + num) % 10;
    }
    res
}
