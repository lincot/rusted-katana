//! <https://www.codewars.com/kata/588e68aed4cff457d300002e/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

#[allow(clippy::missing_const_for_fn)]
pub fn turn(current: char, target: char) -> String {
    match (current, target) {
        ('N', 'W') | ('W', 'S') | ('S', 'E') | ('E', 'N') => "left",
        _ => "right",
    }
    .into()
}
