//! <https://www.codewars.com/kata/5826f54cc60c7e5266000baf/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn minimum_perimeter(area: u64) -> u64 {
    (1..=unsafe { sqrtf64(area as _).to_int_unchecked() })
        .rev()
        .find(|width| area % width == 0)
        .map_or(0, |width| 2 * (area / width + width))
}
