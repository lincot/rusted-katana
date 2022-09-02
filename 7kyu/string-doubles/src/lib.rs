//! <https://www.codewars.com/kata/5a145ab08ba9148dd6000094/train/rust>

use my_prelude::prelude::*;

pub fn doubles(s: &str) -> String {
    let mut chars = Vec::with_capacity(s.len());
    for c in s.chars() {
        if chars.last() == Some(&c) {
            chars.pop();
        } else {
            unsafe { chars.push_unchecked(c) };
        }
    }

    let mut res = String::with_capacity(s.len());
    for c in chars {
        unsafe { res.push_unchecked(c) };
    }
    res
}
