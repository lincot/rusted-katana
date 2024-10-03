//! <https://www.codewars.com/kata/56efc695740d30f963000557/train/rust>

use unchecked_std::prelude::*;

pub fn to_alternating_case(s: &str) -> String {
    let mut res = String::with_capacity(3 * s.len());
    unsafe {
        for ch in s.chars() {
            if ch.is_ascii() {
                if ch.is_ascii_lowercase() {
                    res.push_unchecked(ch.to_ascii_uppercase());
                } else {
                    res.push_unchecked(ch.to_ascii_lowercase());
                }
            } else if ch.is_lowercase() {
                res.extend_unchecked(ch.to_uppercase());
            } else {
                res.extend_unchecked(ch.to_lowercase());
            }
        }
    }
    res
}
