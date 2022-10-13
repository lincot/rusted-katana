//! <https://www.codewars.com/kata/586f6741c66d18c22800010a/train/rust>

#![no_std]

pub const fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    if end < start {
        return 0;
    }
    let n = (end - start) / step + 1;
    n * (2 * start + (n - 1) * step) / 2
}
