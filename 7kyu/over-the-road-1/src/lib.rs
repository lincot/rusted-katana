//! <https://www.codewars.com/kata/5f0ed36164f2bc00283aed07/train/rust>

#![no_std]

pub const fn over_the_road(address: u64, n: u64) -> u64 {
    2 * n - address + 1
}
