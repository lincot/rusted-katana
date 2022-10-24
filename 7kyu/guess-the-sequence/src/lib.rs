//! <https://www.codewars.com/kata/5b45e4b3f41dd36bf9000090/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn sequence(x: u8) -> Vec<u8> {
    let mut res = Vec::with_capacity(x as _);

    if x < 10 {
        for i in 1..=x {
            unsafe { res.push_unchecked(i) };
        }
        return res;
    }

    for d in 1..=9 {
        unsafe { res.push_unchecked(d) };
        if x >= 10 * d {
            for n in 10 * d..=(10 * d + 9).min(x) {
                unsafe { res.push_unchecked(n) };
            }
        }
    }

    if x == 100 {
        if res.len() == res.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        res.insert(2, 100);
    }

    res
}
