//! <https://www.codewars.com/kata/5875b200d520904a04000003/train/rust>

#![no_std]

#[allow(clippy::missing_const_for_fn)]
pub fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    (wait - (cap - on)).max(0)
}
