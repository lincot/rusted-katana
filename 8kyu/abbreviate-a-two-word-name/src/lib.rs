//! <https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use my_prelude::prelude::*;

unsafe fn push_unchecked_uppercase(s: &mut String, c: char) {
    if c.is_lowercase() {
        s.extend_unchecked(c.to_uppercase());
    } else {
        s.push_unchecked(c);
    }
}

pub fn abbrev_name(name: &str) -> String {
    let first = name.chars().next().unwrap();
    let first_len = first.len_utf8();

    let space_pos = name[first_len..].bytes().position(|b| b == b' ').unwrap() + first_len;

    let last = name[space_pos + 1..].chars().next();
    let last = unsafe { last.unwrap_unchecked() };

    let cap = 6 + 1 + 6;
    let mut res = String::with_capacity(cap);

    unsafe {
        push_unchecked_uppercase(&mut res, first);
        res.push_unchecked('.');
        push_unchecked_uppercase(&mut res, last);
    }

    res
}
