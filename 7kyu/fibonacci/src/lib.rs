//! <https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{powif64, sqrtf64};

pub fn fib(n: u32) -> u32 {
    unsafe {
        let s5 = sqrtf64(5.);
        let phi = (1. + s5) / 2.;
        let little_phi = (1. - s5) / 2.;
        ((powif64(phi, n as _) - powif64(little_phi, n as _)) / s5 + 0.000_001) as _
    }
}
