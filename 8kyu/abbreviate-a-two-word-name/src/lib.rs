//! <https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust>

use unchecked_std::prelude::*;

pub fn abbrev_name(name: &str) -> String {
    let first = name.chars().next().unwrap();
    let first_len = first.len_utf8();

    let space_pos = name[first_len..].bytes().position(|b| b == b' ').unwrap() + first_len;

    let last = name[space_pos + 1..].chars().next();
    let last = unsafe { last.unwrap_unchecked() };

    let cap = 6 + 1 + 6;
    let mut res = String::with_capacity(cap);

    unsafe {
        push_uppercase_unchecked(&mut res, first);
        res.push_unchecked('.');
        push_uppercase_unchecked(&mut res, last);
    }

    res
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_uppercase() {
        s.push_unchecked(ch);
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}
