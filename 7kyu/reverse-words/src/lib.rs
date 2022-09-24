//! <https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

pub fn reverse_words(str: &str) -> String {
    let mut res = String::with_capacity(str.len());

    let mut reversed_words = str.as_bytes().split(|&b| b == b' ').map(|bytes| {
        unsafe { core::str::from_utf8_unchecked(bytes) }
            .chars()
            .rev()
    });

    if let Some(w) = reversed_words.next() {
        unsafe { res.extend_unchecked(w) };
    }
    for w in reversed_words {
        unsafe { res.push_unchecked(' ') };
        unsafe { res.extend_unchecked(w) };
    }

    res
}
