//! <https://www.codewars.com/kata/5b06c990908b7eea73000069/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::logf64;
use num_complex::Complex;

fn log(a: f64, b: f64) -> f64 {
    unsafe { logf64(a) / logf64(b) }
}

pub fn f(z: Complex<f64>, eps: f64) -> i32 {
    let norm = z.norm();
    if norm >= 1. {
        -1
    } else {
        log(eps, norm) as _
    }
}
