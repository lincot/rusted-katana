//! <https://www.codewars.com/kata/5963c18ecb97be020b0000a2/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn derive(coefficient: u32, exponent: u32) -> String {
    let mut res = String::with_capacity(10 + 2 + 10);
    unsafe {
        res.write_num_unchecked(coefficient * exponent);
        res.push_str_unchecked("x^");
        res.write_num_unchecked(exponent - 1);
    }
    res
}
