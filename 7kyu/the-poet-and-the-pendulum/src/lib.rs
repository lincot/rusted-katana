//! <https://www.codewars.com/kata/5bd776533a7e2720c40000e5/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use prelude::*;

pub fn pendulum(xs: &[i32]) -> Vec<i32> {
    let mut xs = xs.to_vec();
    xs.sort_unstable();
    let mut res = Vec::with_capacity(xs.len());
    let mut i = (xs.len() - (xs.len() % 2 == 0) as usize).wrapping_sub(1);
    for _ in 0..(i + 2) / 2 {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
        i = i.wrapping_sub(2);
    }
    let mut i = 1;
    for _ in 0..xs.len() / 2 {
        unsafe { res.push_unchecked(*xs.get_unchecked(i)) };
        i += 2;
    }
    res
}
