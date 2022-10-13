//! <https://www.codewars.com/kata/628e3ee2e1daf90030239e8a/train/rust>

#![no_std]

pub const fn interlockable(a: u64, b: u64) -> bool {
    a & b == 0
}
