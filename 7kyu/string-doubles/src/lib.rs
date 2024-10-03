//! <https://www.codewars.com/kata/5a145ab08ba9148dd6000094/train/rust>

use unchecked_std::prelude::*;

pub fn doubles(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(doubles_bytes(s.as_bytes())) };
    }

    let mut chars = Vec::with_capacity(s.len());
    for ch in s.chars() {
        if chars.last() == Some(&ch) {
            chars.pop();
        } else {
            unsafe { chars.push_unchecked(ch) };
        }
    }

    let mut res = String::with_capacity(s.len());
    for ch in chars {
        unsafe { res.push_unchecked(ch) };
    }
    res
}

fn doubles_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(s.len());
    for &b in s {
        if res.last() == Some(&b) {
            res.pop();
        } else {
            unsafe { res.push_unchecked(b) };
        }
    }
    res
}
