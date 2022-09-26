//! <https://www.codewars.com/kata/57aa218e72292d98d500240f/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn heron(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.;
    unsafe { sqrtf64(s * (s - a) * (s - b) * (s - c)) }
}
