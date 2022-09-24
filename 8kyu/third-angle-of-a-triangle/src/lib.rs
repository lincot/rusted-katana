//! <https://www.codewars.com/kata/5a023c426975981341000014/train/rust>

#![no_std]

pub const fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}
