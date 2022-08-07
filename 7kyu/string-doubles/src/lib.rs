//! <https://www.codewars.com/kata/5a145ab08ba9148dd6000094/train/rust>

use my_prelude::prelude::*;

pub fn doubles(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());

    for c in s.chars() {
        if res.last() == Some(&c) {
            res.pop();
        } else {
            unsafe { res.push_unchecked(c) };
        }
    }

    res.into_iter().collect()
}
