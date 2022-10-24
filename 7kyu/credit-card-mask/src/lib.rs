//! <https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn maskify(cc: &str) -> String {
    let mut shown_chars_count = 0;
    let mut shown_chars_len = 0;
    let mut hidden_chars_count = 0;
    for c in cc.chars().rev() {
        if shown_chars_count < 4 {
            shown_chars_len += c.len_utf8();
            shown_chars_count += 1;
        } else {
            hidden_chars_count += 1;
        }
    }

    let mut res = String::with_capacity(hidden_chars_count + shown_chars_len);
    unsafe {
        for _ in 0..hidden_chars_count {
            res.push_unchecked('#');
        }
        res.push_str_unchecked(cc.get_unchecked(cc.len() - shown_chars_len..));
    }
    res
}
