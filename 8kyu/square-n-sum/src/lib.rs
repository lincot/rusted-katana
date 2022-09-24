//! <https://www.codewars.com/kata/515e271a311df0350d00000f/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn square_sum(vec: Vec<i32>) -> i32 {
    vec.into_iter().map(|x| x * x).sum()
}
