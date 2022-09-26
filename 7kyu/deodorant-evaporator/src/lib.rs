//! <https://www.codewars.com/kata/5506b230a11c0aeab3000c1f/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::logf64;

fn log(a: f64, b: f64) -> f64 {
    unsafe { logf64(a) / logf64(b) }
}

pub fn evaporator(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    (log(threshold as f64 * 0.01, 1.0 - evap_per_day as f64 * 0.01) + 0.999_999_99) as _
}
