//! <https://www.codewars.com/kata/58ce8725c835848ad6000007/train/rust>

#![no_std]

pub const fn potatoes(p0: i64, w0: i64, p1: i64) -> i64 {
    w0 * (100 - p0) / (100 - p1)
}
