//! <https://www.codewars.com/kata/56269eb78ad2e4ced1000013/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

pub fn find_next_square(sq: u64) -> Option<u64> {
    perfect_sqrt(sq).map(|x| (x + 1).pow(2))
}

fn perfect_sqrt(n: u64) -> Option<u64> {
    if [0, 1, 4, 9].contains(&(n & 0xf)) {
        let s = unsafe { sqrtf64(n as _) } as _;
        if s * s == n {
            Some(s)
        } else {
            None
        }
    } else {
        None
    }
}
