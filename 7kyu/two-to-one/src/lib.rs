//! <https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust>

use unchecked_std::prelude::*;

pub fn longest(a1: &str, a2: &str) -> String {
    let mut letters = [false; 26];
    for a in [a1, a2] {
        for b in a.bytes() {
            if b.is_ascii_lowercase() {
                letters[(b - b'a') as usize] = true;
            }
        }
    }

    let mut res = Vec::with_capacity(26);
    for i in 0..26 {
        if letters[i as usize] {
            unsafe { res.push_unchecked(b'a' + i) };
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
