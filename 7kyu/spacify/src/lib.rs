//! <https://www.codewars.com/kata/57f8ee485cae443c4d000127/train/rust>

use unchecked_std::prelude::*;

pub fn spacify(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(spacify_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(2 * s.len());
    let mut s = s.chars();
    if let Some(ch) = s.next() {
        unsafe { res.push_unchecked(ch) };
    }
    for ch in s {
        unsafe {
            res.push_unchecked(' ');
            res.push_unchecked(ch);
        }
    }
    res
}

fn spacify_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(2 * s.len());
    let mut s = s.iter();
    if let Some(&b) = s.next() {
        unsafe { res.push_unchecked(b) };
    }
    for &b in s {
        unsafe {
            res.push_unchecked(b' ');
            res.push_unchecked(b);
        }
    }
    res
}
