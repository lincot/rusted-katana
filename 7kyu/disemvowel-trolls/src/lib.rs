//! <https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust>

use my_prelude::prelude::*;

pub fn disemvowel(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());
    for b in s.bytes() {
        if !b"eaiouEAIOU".contains(&b) {
            unsafe { res.push_unchecked(b) };
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
