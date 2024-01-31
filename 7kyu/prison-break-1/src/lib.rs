//! <https://www.codewars.com/kata/6507e3170b7009117e0c7865/train/rust>

#![feature(array_windows)]

pub fn freed_prisoners(prison: &[bool]) -> u32 {
    if prison.first() == Some(&true) {
        prison.array_windows().filter(|[a, b]| a != b).count() as _
    } else {
        0
    }
}
