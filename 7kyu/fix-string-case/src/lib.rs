//! <https://www.codewars.com/kata/5b180e9fedaa564a7000009a/train/rust>

use unchecked_std::prelude::*;

pub fn solve(s: &str) -> String {
    let mut chars = Vec::with_capacity(s.len());
    for ch in s.chars() {
        unsafe { chars.push_unchecked((ch, ch.is_lowercase(), ch.is_uppercase())) };
    }

    let lowercase_count = chars
        .iter()
        .filter(|&&(_, is_lowercase, _)| is_lowercase)
        .count();
    let uppercase_count = chars
        .iter()
        .filter(|&&(_, _, is_uppercase)| is_uppercase)
        .count();

    let mut res = String::with_capacity(3 * s.len());

    if uppercase_count > lowercase_count {
        for &(ch, _, is_uppercase) in &chars {
            if is_uppercase {
                unsafe { res.push_unchecked(ch) };
            } else if ch.is_ascii() {
                unsafe { res.push_unchecked(ch.to_ascii_uppercase()) };
            } else {
                unsafe { res.extend_unchecked(ch.to_uppercase()) };
            }
        }
    } else {
        for &(ch, is_lowercase, _) in &chars {
            if is_lowercase {
                unsafe { res.push_unchecked(ch) };
            } else if ch.is_ascii() {
                unsafe { res.push_unchecked(ch.to_ascii_lowercase()) };
            } else {
                unsafe { res.extend_unchecked(ch.to_lowercase()) };
            }
        }
    }

    res
}
