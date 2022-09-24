//! <https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn odd_or_even(numbers: Vec<i32>) -> String {
    if numbers.into_iter().sum::<i32>() % 2 == 0 {
        "even"
    } else {
        "odd"
    }
    .into()
}
