//! <https://www.codewars.com/kata/56453a12fcee9a6c4700009c/train/rust>

#![no_std]

pub fn close_compare(a: f64, b: f64, margin: f64) -> i8 {
    if a < b - margin {
        -1
    } else {
        (a > b + margin) as _
    }
}
