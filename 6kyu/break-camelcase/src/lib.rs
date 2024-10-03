//! <https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust>

use unchecked_std::prelude::*;

pub fn solution(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(solution_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(2 * s.len());
    for ch in s.chars() {
        if ch.is_uppercase() {
            unsafe { res.push_unchecked(' ') };
        }
        unsafe { res.push_unchecked(ch) };
    }
    res
}

fn solution_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(2 * s.len());
    for &b in s {
        if b.is_ascii_uppercase() {
            unsafe { res.push_unchecked(b' ') };
        }
        unsafe { res.push_unchecked(b) };
    }
    res
}
