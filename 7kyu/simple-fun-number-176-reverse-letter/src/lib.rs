//! <https://www.codewars.com/kata/58b8c94b7df3f116eb00005b/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use unchecked::PushUnchecked;

pub fn reverse_letters(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for c in s.chars().rev() {
        if c.is_alphabetic() {
            unsafe { res.push_unchecked(c) };
        }
    }
    res
}
