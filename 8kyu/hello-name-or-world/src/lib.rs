//! <https://www.codewars.com/kata/57e3f79c9cb119374600046b/train/rust>

use unchecked_std::prelude::*;

pub fn hello(name: &str) -> String {
    const HELLO: &str = "Hello, ";
    let mut name_chars = name.chars();

    let Some(first) = name_chars.next() else {
        return "Hello, World!".into();
    };

    let cap = HELLO.len() + '!'.len_utf8()
        // reserve for the worst case "ΐ" -> "Ϊ́"
        + 3
        // "İİİİ" -> "i̇i̇i̇i̇" grows 1.5x
        + name.len() + name.len() / 2;
    let mut res = String::with_capacity(cap);

    unsafe { res.push_str_unchecked(HELLO) };
    unsafe { push_uppercase_unchecked(&mut res, first) };
    for ch in name_chars {
        unsafe { push_lowercase_unchecked(&mut res, ch) };
    }
    unsafe { res.push_unchecked('!') };

    res
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}

unsafe fn push_lowercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_lowercase());
    } else if ch.is_lowercase() {
        s.push_unchecked(ch);
    } else {
        s.extend_unchecked(ch.to_lowercase());
    }
}
