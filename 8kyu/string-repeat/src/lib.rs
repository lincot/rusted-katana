//! <https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}
