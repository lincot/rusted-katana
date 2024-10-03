//! <https://www.codewars.com/kata/580755730b5a77650500010c/train/rust>

use unchecked_std::prelude::*;

pub fn sort_my_string(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(sort_my_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(s.len() + 1);
    let mut odd = String::with_capacity(s.len());
    unsafe {
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                res.push_unchecked(c);
            } else {
                odd.push_unchecked(c);
            }
        }
        res.push_unchecked(' ');
        res.push_str_unchecked(&odd);
    }
    res
}

fn sort_my_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(s.len() + 1);
    let mut odd = Vec::with_capacity(s.len() / 2);
    unsafe {
        for (i, &b) in s.iter().enumerate() {
            if i % 2 == 0 {
                res.push_unchecked(b);
            } else {
                odd.push_unchecked(b);
            }
        }
        res.push_unchecked(b' ');
        res.extend_from_slice_unchecked(&odd);
    }
    res
}
