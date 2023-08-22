//! <https://www.codewars.com/kata/559d2284b5bb6799e9000047/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use prelude::*;

pub fn add_length(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len() / 2 + 1);
    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|word| unsafe { core::str::from_utf8_unchecked(word) })
    {
        let mut word_with_length = String::with_capacity(word.len() + 1 + USIZE_MAX_LEN);
        unsafe {
            word_with_length.push_str_unchecked(word);
            word_with_length.push_unchecked(' ');
            word_with_length.write_num_unchecked(word.len(), false, false);
            res.push_unchecked(word_with_length);
        }
    }
    res
}
