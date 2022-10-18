//! <https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn are_you_playing_banjo(name: &str) -> String {
    let text = if b"Rr".contains(&name.bytes().next().unwrap()) {
        " plays banjo"
    } else {
        " does not play banjo"
    };
    let mut res = String::with_capacity(name.len() + text.len());
    unsafe {
        res.push_str_unchecked(name);
        res.push_str_unchecked(text);
    }
    res
}
