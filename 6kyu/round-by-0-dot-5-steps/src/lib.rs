//! <https://www.codewars.com/kata/51f1342c76b586046800002a/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::roundf64;

pub fn solution(n: f64) -> f64 {
    unsafe { roundf64(2. * n) / 2. }
}
