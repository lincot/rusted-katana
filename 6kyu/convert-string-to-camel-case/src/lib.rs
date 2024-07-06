//! <https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust>

use unchecked_std::prelude::*;

pub fn to_camel_case(text: &str) -> String {
    let mut res = String::with_capacity(2 * text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        unsafe {
            if c == '-' || c == '_' {
                if let Some(next) = chars.next() {
                    res.extend_unchecked(next.to_uppercase());
                }
            } else {
                res.push_unchecked(c);
            }
        }
    }
    res
}
