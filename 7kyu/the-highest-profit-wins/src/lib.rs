//! <https://www.codewars.com/kata/559590633066759614000063/train/rust>

#![no_std]

pub fn min_max(lst: &[i32]) -> (i32, i32) {
    lst.iter().fold((i32::MAX, i32::MIN), |(min, max), &new| {
        (min.min(new), max.max(new))
    })
}
