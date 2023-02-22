//! <https://www.codewars.com/kata/5898761a9c700939ee000011/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::{
    f64::consts::SQRT_2,
    intrinsics::{powif64, roundf64},
};

pub fn corner_circle(r: f64) -> f64 {
    unsafe {
        let c = powif64(SQRT_2 - 1., 2);
        roundf64(100. * c * r) / 100.
    }
}
