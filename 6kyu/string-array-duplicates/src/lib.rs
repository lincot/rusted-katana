//! <https://www.codewars.com/kata/59f08f89a5e129c543000069/train/rust>

use my_prelude::prelude::*;

pub fn dup(mut arry: Vec<String>) -> Vec<String> {
    for s in &mut arry {
        let mut res = String::with_capacity(s.len());

        let mut prev = if let Some(c) = s.chars().next() {
            c
        } else {
            continue;
        };
        unsafe { res.push_unchecked(prev) };
        for c in s.chars() {
            if c != prev {
                unsafe { res.push_unchecked(c) };
            }
            prev = c;
        }

        *s = res;
    }

    arry
}
