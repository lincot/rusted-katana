//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>

#![no_std]

pub const fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}
