//! <https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust>

use unchecked_std::prelude::*;

pub fn to_camel_case(text: &str) -> String {
    if text.is_ascii() {
        return unsafe { String::from_utf8_unchecked(to_camel_case_bytes(text.as_bytes())) };
    }

    let mut res = String::with_capacity(2 * text.len());
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        unsafe {
            if ch == '-' || ch == '_' {
                if let Some(next) = chars.next() {
                    res.extend_unchecked(next.to_uppercase());
                }
            } else {
                res.push_unchecked(ch);
            }
        }
    }
    res
}

fn to_camel_case_bytes(text: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(text.len());
    let mut bytes = text.iter();
    while let Some(&b) = bytes.next() {
        unsafe {
            if b == b'-' || b == b'_' {
                if let Some(next) = bytes.next() {
                    res.push_unchecked(next.to_ascii_uppercase());
                }
            } else {
                res.push_unchecked(b);
            }
        }
    }
    res
}
