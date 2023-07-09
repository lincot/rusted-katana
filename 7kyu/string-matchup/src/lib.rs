//! <https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    let mut res = Vec::with_capacity(a2.len());
    unsafe { res.set_len(a2.len()) };
    for (r, s2) in res.iter_mut().zip(a2) {
        *r = a1.iter().filter(|&s1| s1 == s2).count();
    }
    res
}
