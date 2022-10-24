//! <https://www.codewars.com/kata/56b8903933dbe5831e000c76/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn spoonerize(words: &str) -> String {
    let first_len = words.chars().next().unwrap().len_utf8();
    let second_pos = words.bytes().position(|b| b == b' ').unwrap() + 1;
    assert!(second_pos != 1);
    let second_len = words[second_pos..].chars().next().unwrap().len_utf8();

    let mut res = String::with_capacity(words.len());

    unsafe {
        res.push_str_unchecked(words.get_unchecked(second_pos..second_pos + second_len));
        res.push_str_unchecked(words.get_unchecked(first_len..second_pos));
        res.push_str_unchecked(words.get_unchecked(..first_len));
        res.push_str_unchecked(words.get_unchecked(second_pos + second_len..));
    }

    res
}
