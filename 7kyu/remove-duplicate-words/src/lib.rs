//! <https://www.codewars.com/kata/5b39e3772ae7545f650000fc/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use my_prelude::prelude::*;

pub fn remove_duplicate_words(s: &str) -> String {
    let mut words = Vec::with_capacity(s.len() / 2 + 1);
    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|bytes| unsafe { core::str::from_utf8_unchecked(bytes) })
    {
        if !words.contains(&word) {
            unsafe { words.push_unchecked(word) };
        }
    }

    let mut res = String::with_capacity(s.len());
    if let Some(first) = words.first() {
        unsafe { res.push_str_unchecked(first) };
    }
    for word in &words[1..] {
        unsafe {
            res.push_unchecked(' ');
            res.push_str_unchecked(word);
        }
    }
    res
}
