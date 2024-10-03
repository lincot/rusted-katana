//! <https://www.codewars.com/kata/535474308bb336c9980006f2/train/rust>

use unchecked_std::prelude::*;

pub fn greet(name: &str) -> String {
    let mut res = String::with_capacity("Hello !".len() + 3 + name.len() + name.len() / 2);
    unsafe {
        res.push_str_unchecked("Hello ");
        let mut name = name.chars();
        if let Some(ch) = name.next() {
            push_uppercase_unchecked(&mut res, ch);
        }
        for ch in name {
            push_lowercase_unchecked(&mut res, ch);
        }
        res.push_unchecked('!');
    }
    res
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else if ch.is_uppercase() {
        s.push_unchecked(ch);
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
