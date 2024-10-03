//! <https://www.codewars.com/kata/59cfc000aeb2844d16000075/train/rust>

use core::mem::swap;
use unchecked_std::prelude::*;

pub fn capitalize(s: &str) -> Vec<String> {
    let cap = 2 * s.len() + s.len() / 3;
    let mut first = String::with_capacity(cap);
    let mut second = String::with_capacity(cap);

    let mut chars_count = 0usize;

    for ch in s.chars() {
        unsafe {
            push_uppercase_unchecked(&mut first, ch);
            second.push_unchecked(ch);
        }

        swap(&mut first, &mut second);

        chars_count += 1;
    }
    if chars_count % 2 == 1 {
        swap(&mut first, &mut second);
    }

    vec![first, second]
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}
