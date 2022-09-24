//! <https://www.codewars.com/kata/5265326f5fda8eb1160004c8/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use lexical::to_string;

#[allow(non_upper_case_globals)]
pub const number_to_string: fn(i32) -> String = to_string;
