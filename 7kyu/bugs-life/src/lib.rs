//! <https://www.codewars.com/kata/5b71af678adeae41df00008c/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{fmaf64, sqrtf64};

pub fn shortest_distance(a: f64, b: f64, c: f64) -> f64 {
    let largest = a.max(b).max(c);
    let sum_others = a + b + c - largest;
    unsafe { sqrtf64(fmaf64(largest, largest, sum_others * sum_others)) }
}
