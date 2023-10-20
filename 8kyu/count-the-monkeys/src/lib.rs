//! <https://www.codewars.com/kata/56f69d9f9400f508fb000ba7/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn monkey_count(n: i32) -> Vec<i32> {
    (1..=n).collect()
}
