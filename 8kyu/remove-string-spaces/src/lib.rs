//! <https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn no_space(mut x: String) -> String {
    let vec = unsafe { x.as_mut_vec() };
    vec.retain(|&c| c != b' ');
    x
}
