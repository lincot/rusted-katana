//! <https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust>

use my_prelude::prelude::*;

pub fn maskify(cc: &str) -> String {
    let mut shown_chars_count = 0;
    let mut shown_chars_len = 0;
    let mut hidden_chars_count = 0;
    for c in cc.chars().rev() {
        if shown_chars_count < 4 {
            shown_chars_len += c.len_utf8();
            shown_chars_count += 1;
        } else {
            hidden_chars_count += 1;
        }
    }

    let mut res = Vec::with_capacity(hidden_chars_count + shown_chars_len);
    unsafe {
        for _ in 0..hidden_chars_count {
            res.push_unchecked(b'#');
        }
        res.extend_from_slice_unchecked(cc.as_bytes().get_unchecked(cc.len() - shown_chars_len..));
    }
    unsafe { String::from_utf8_unchecked(res) }
}
