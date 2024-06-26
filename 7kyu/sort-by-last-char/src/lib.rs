//! <https://www.codewars.com/kata/57eba158e8ca2c8aba0002a0/train/rust>

use core::hint::unreachable_unchecked;
use unchecked_std::prelude::*;

pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len() / 2 + 1);

    for word in s.as_bytes().split(|&b| b == b' ') {
        if !word.is_empty() {
            unsafe { res.push_unchecked(String::from_utf8_unchecked(word.to_vec())) };
        }
    }

    let key_fn = |s: &String| {
        if s.is_empty() {
            unsafe { unreachable_unchecked() };
        }
        s.as_bytes()[s.len() - 1]
    };
    if res.len() <= 20 {
        res.sort_by_key(key_fn);
    } else {
        radsort::sort_by_key(&mut res, key_fn);
    }

    res
}
