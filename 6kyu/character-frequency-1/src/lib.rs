//! <https://www.codewars.com/kata/53e895e28f9e66a56900011a/train/rust>

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;

pub fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();
    for c in input.chars().filter(|c| c.is_alphabetic()) {
        if c.is_lowercase() {
            *res.entry(c).or_insert(0) += 1;
        } else {
            for c in c.to_lowercase() {
                *res.entry(c).or_insert(0) += 1;
            }
        }
    }
    res
}
