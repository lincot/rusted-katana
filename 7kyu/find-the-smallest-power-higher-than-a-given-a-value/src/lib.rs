//! <https://www.codewars.com/kata/56ba65c6a15703ac7e002075/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::powf64;

pub fn find_next_power(val: u64, pow_: u32) -> u64 {
    let base = unsafe { powf64(val as _, 1. / pow_ as f64) } as u64 + 1;
    let res = base.pow(pow_);
    if res > val {
        res
    } else {
        (base + 1).pow(pow_)
    }
}
