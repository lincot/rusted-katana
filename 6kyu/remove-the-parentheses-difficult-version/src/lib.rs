//! <https://www.codewars.com/kata/62f39741ec164f2b28c257c5/train/rust>

use unchecked_std::prelude::*;

pub fn remove_parentheses(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());
    let mut depth = 0;
    for b in s.bytes() {
        unsafe { res.push_unchecked(b) };
        if b == b'(' {
            depth += 1;
        } else if b == b')' && depth > 0 {
            depth -= 1;
            while res.pop() != Some(b'(') {}
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
