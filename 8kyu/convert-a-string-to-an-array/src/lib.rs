//! <https://www.codewars.com/kata/57e76bc428d6fbc2d500036d/train/rust>

use unchecked_std::prelude::*;

pub fn string_to_array(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len() / 2 + 1);
    for word in s.as_bytes().split(|&b| b == b' ') {
        unsafe { res.push_unchecked(String::from_utf8_unchecked(word.to_vec())) };
    }
    res
}
