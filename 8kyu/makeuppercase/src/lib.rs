//! <https://www.codewars.com/kata/57a0556c7cb1f31ab3000ad7/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

#[allow(non_upper_case_globals)]
pub const make_upper_case: fn(&str) -> String = str::to_uppercase;
