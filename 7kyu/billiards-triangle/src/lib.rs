//! <https://www.codewars.com/kata/5bb3e299484fcd5dbb002912/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::{fmaf64, sqrtf64};

pub fn pyramid(balls: u16) -> u16 {
    ((unsafe { sqrtf64(fmaf64(8.0f64, balls as _, 1.)) } - 1.) / 2.) as _
}
