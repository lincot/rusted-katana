//! <https://www.codewars.com/kata/587731fda577b3d1b0001196/train/rust>

use unchecked_std::prelude::*;

pub fn camel_case(str: &str) -> String {
    if str.is_ascii() {
        return unsafe { String::from_utf8_unchecked(camel_case_bytes(str.as_bytes())) };
    }

    let mut res = String::with_capacity(2 * str.len() + str.len() / 3);

    let mut chars = str.chars();

    for ch in chars.by_ref() {
        if ch != ' ' {
            unsafe { res.extend_unchecked(ch.to_uppercase()) };
            break;
        }
    }
    while let Some(ch) = chars.next() {
        if ch == ' ' {
            for ch in chars.by_ref() {
                if ch != ' ' {
                    unsafe { res.extend_unchecked(ch.to_uppercase()) };
                    break;
                }
            }
        } else {
            unsafe { res.push_unchecked(ch) };
        }
    }

    res
}

fn camel_case_bytes(s: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(s.len());

    let mut bytes = s.iter();

    for &b in bytes.by_ref() {
        if b != b' ' {
            unsafe { res.push_unchecked(b.to_ascii_uppercase()) };
            break;
        }
    }
    while let Some(&b) = bytes.next() {
        if b == b' ' {
            for &b in bytes.by_ref() {
                if b != b' ' {
                    unsafe { res.push_unchecked(b.to_ascii_uppercase()) };
                    break;
                }
            }
        } else {
            unsafe { res.push_unchecked(b) };
        }
    }

    res
}
