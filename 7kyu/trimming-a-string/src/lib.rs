//! <https://www.codewars.com/kata/563fb342f47611dae800003c/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn trim(phrase: &str, length: usize) -> String {
    if phrase.len() <= length {
        return phrase.into();
    }
    let length = if length > 3 { length - 3 } else { length };
    for (i, (ci, _)) in phrase.char_indices().enumerate() {
        if i == length {
            let mut res = String::with_capacity(ci + 3);
            unsafe {
                res.push_str_unchecked(phrase.get_unchecked(..ci));
                res.push_str_unchecked("...");
            }
            return res;
        }
    }
    phrase.into()
}
