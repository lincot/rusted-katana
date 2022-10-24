//! <https://www.codewars.com/kata/57e3f79c9cb119374600046b/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

unsafe fn push_unchecked_uppercase(s: &mut String, c: char) {
    if c.is_lowercase() {
        s.extend_unchecked(c.to_uppercase());
    } else {
        s.push_unchecked(c);
    }
}

unsafe fn push_unchecked_lowercase(s: &mut String, c: char) {
    if c.is_lowercase() {
        s.push_unchecked(c);
    } else {
        s.extend_unchecked(c.to_lowercase());
    }
}

pub fn hello(name: &str) -> String {
    const HELLO: &str = "Hello, ";
    let mut name_chars = name.chars();

    let first = match name_chars.next() {
        Some(c) => c,
        None => return "Hello, World!".into(),
    };

    let cap = HELLO.len() + '!'.len_utf8()
        // reserve for worst case "ΐ" -> "Ϊ́"
        + 3
        // "İİİİ" -> "i̇i̇i̇i̇" growing 1.5x
        + name.len() + name.len() / 2;
    let mut res = String::with_capacity(cap);

    unsafe { res.push_str_unchecked(HELLO) };
    unsafe { push_unchecked_uppercase(&mut res, first) };
    for c in name_chars {
        unsafe { push_unchecked_lowercase(&mut res, c) };
    }
    unsafe { res.push_unchecked('!') };

    res
}
