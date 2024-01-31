//! <https://www.codewars.com/kata/59c053f070a3b7d19100007e/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{fabsf64, fmaf64, sqrtf64};

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    unsafe { sqrtf64((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) }
}

pub fn distance_from_line(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    unsafe {
        if a == b {
            dist(a, c)
        } else {
            fabsf64(fmaf64(a.0 - b.0, b.1 - c.1, (c.0 - b.0) * (a.1 - b.1))) / dist(a, b)
        }
    }
}
