//! <https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};

pub fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    let mut res = vec![0; a2.len()];
    for (i, s2) in a2.iter().enumerate() {
        for s1 in a1 {
            if s1 == s2 {
                unsafe { *res.get_unchecked_mut(i) += 1 };
            }
        }
    }
    res
}
