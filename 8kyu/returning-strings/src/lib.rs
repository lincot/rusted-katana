//! <https://www.codewars.com/kata/55a70521798b14d4750000a4/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn greet(name: &str) -> String {
    let mut res = String::with_capacity("Hello,  how are you doing today?".len() + name.len());
    unsafe {
        res.push_str_unchecked("Hello, ");
        res.push_str_unchecked(name);
        res.push_str_unchecked(" how are you doing today?");
    }
    res
}
