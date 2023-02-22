//! <https://www.codewars.com/kata/589896b99c70093f3e00005b/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::roundf32;

pub fn sixteen_circles(r: usize) -> f32 {
    unsafe { roundf32(r as f32 * 114.626_437) / 100. }
}
