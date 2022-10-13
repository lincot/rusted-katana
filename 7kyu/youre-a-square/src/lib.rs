//! <https://www.codewars.com/kata/54c27a33fb7da0db0100040e/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn is_square(n: i64) -> bool {
    n >= 0 && [0, 1, 4, 9].contains(&(n & 0xf)) && {
        let s = unsafe { sqrtf64(n as _) } as i64;
        s * s == n
    }
}
