//! <https://www.codewars.com/kata/570523c146edc287a50014b1/train/rust>

#![no_std]

pub fn number_joy(n: u32) -> bool {
    [1, 81, 1458, 1729].contains(&n)
}
