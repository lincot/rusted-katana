//! <https://www.codewars.com/kata/54162d1333c02486a700011d/train/rust>

#![no_std]

pub const fn penultimate(a: &[i32]) -> i32 {
    a[a.len() - 2]
}
