//! <https://www.codewars.com/kata/56f69d9f9400f508fb000ba7/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn monkey_count(n: i32) -> Vec<i32> {
    let n = n.max(0);
    let mut res = Vec::with_capacity(n as _);
    unsafe { res.set_len(n as _) };
    for (i, r) in (0..).zip(res.iter_mut()) {
        *r = i + 1;
    }
    res
}
