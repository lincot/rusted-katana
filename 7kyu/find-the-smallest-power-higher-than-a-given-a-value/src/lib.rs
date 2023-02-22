//! <https://www.codewars.com/kata/56ba65c6a15703ac7e002075/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::powf64;

pub fn find_next_power(val: u64, pow_: u32) -> u64 {
    let base: u64 = unsafe { (powf64(val as _, 1. / pow_ as f64) + 1.000_001).to_int_unchecked() };
    base.pow(pow_)
}
