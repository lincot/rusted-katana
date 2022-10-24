//! <https://www.codewars.com/kata/57fd696e26b06857eb0011e7/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn dative(word: &str) -> String {
    let mut res = String::with_capacity(word.len() + 3);
    unsafe { res.push_str_unchecked(word) };
    for b in word.bytes().rev() {
        match b {
            b'e' | 0xa9 | b'i' | 0xad | 0xb6 | 0x91 | 0xbc | 0xb1 => {
                unsafe { res.push_str_unchecked("nek") };
                break;
            }
            b'a' | 0xa1 | b'o' | 0xb3 | b'u' | 0xba => {
                unsafe { res.push_str_unchecked("nak") };
                break;
            }
            _ => {}
        }
    }
    res
}
