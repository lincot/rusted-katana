//! <https://www.codewars.com/kata/566be96bb3174e155300001b/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::fmaf64;

pub fn max_ball(v0: i32) -> i32 {
    unsafe { fmaf64(v0 as f64, 10. / 3.6 / 9.81, 0.5).to_int_unchecked() }
}
