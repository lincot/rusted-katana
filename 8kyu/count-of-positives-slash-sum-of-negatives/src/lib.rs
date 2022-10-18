//! <https://www.codewars.com/kata/576bb71bbbcf0951d5000044/train/rust>

#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return input;
    }
    let (count, num) = input.into_iter().fold((0, 0), |(count, num), x| {
        if x > 0 {
            (count + 1, num)
        } else {
            (count, num + x)
        }
    });
    vec![count, num]
}
