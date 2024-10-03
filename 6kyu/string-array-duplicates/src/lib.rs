//! <https://www.codewars.com/kata/59f08f89a5e129c543000069/train/rust>

use unchecked_std::prelude::*;

pub fn dup(mut arry: Vec<String>) -> Vec<String> {
    for s in &mut arry {
        let mut res = String::with_capacity(s.len());

        let Some(mut prev) = s.chars().next() else {
            continue;
        };
        unsafe { res.push_unchecked(prev) };
        for ch in s.chars() {
            if ch != prev {
                unsafe { res.push_unchecked(ch) };
            }
            prev = ch;
        }

        *s = res;
    }

    arry
}
