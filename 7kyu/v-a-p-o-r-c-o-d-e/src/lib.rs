//! <https://www.codewars.com/kata/5966eeb31b229e44eb00007a/train/rust>

use unchecked_std::prelude::*;

pub fn vaporcode(s: &str) -> String {
    if s.is_ascii() {
        return unsafe { String::from_utf8_unchecked(vaporcode_bytes(s.as_bytes())) };
    }

    let mut res = String::with_capacity(3 * s.len());
    let mut not_first = false;
    for ch in s.chars() {
        if ch != ' ' {
            unsafe {
                if not_first {
                    res.push_str_unchecked("  ");
                } else {
                    not_first = true;
                }
                push_uppercase_unchecked(&mut res, ch);
            }
        }
    }
    res
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}

fn vaporcode_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(3 * s.len());
    let mut not_first = false;
    for &b in s {
        if b != b' ' {
            unsafe {
                if not_first {
                    res.extend_from_slice_unchecked(b"  ");
                } else {
                    not_first = true;
                }
                res.push_unchecked(b.to_ascii_uppercase());
            }
        }
    }
    res
}
