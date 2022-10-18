//! <https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

pub fn remove_char(s: &str) -> String {
    let mut chars = s.chars();
    let first_len = chars.next().unwrap().len_utf8();
    let last_len = chars.rev().next().unwrap().len_utf8();
    unsafe { s.get_unchecked(first_len..s.len() - last_len) }.into()
}
