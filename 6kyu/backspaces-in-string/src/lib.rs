//! <https://www.codewars.com/kata/5727bb0fe81185ae62000ae3/train/rust>

use unchecked_std::prelude::*;

pub fn clean_string(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if b == b'#' {
            res.pop();
        } else {
            unsafe { res.as_mut_vec().push_unchecked(b) };
        }
    }
    res
}
