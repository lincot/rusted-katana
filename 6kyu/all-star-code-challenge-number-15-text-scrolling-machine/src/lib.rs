//! <https://www.codewars.com/kata/586560a639c5ab3a260000f3/train/rust>

use unchecked_std::prelude::*;

pub fn rotate(s: &str) -> Vec<String> {
    let mut processed_len = 0;
    let mut res = Vec::with_capacity(s.len());
    while processed_len != s.len() {
        let mut shifted_s = String::with_capacity(s.len());
        let source = res.last().map_or(s, String::as_str);
        unsafe {
            let c = source.chars().next().unwrap_unchecked();
            shifted_s.push_str_unchecked(source.get_unchecked(c.len_utf8()..));
            shifted_s.push_unchecked(c);
            res.push_unchecked(shifted_s);
            processed_len += c.len_utf8();
        };
    }
    res
}
