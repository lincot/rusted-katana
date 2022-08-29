//! <https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust>

use my_prelude::prelude::*;

pub fn maskify(cc: &str) -> String {
    let chars: Vec<_> = cc.chars().collect();

    let hidden_chars = chars.len().saturating_sub(4);
    let last_4_chars_len: usize = chars.iter().rev().take(4).map(|c| c.len_utf8()).sum();

    let mut res = Vec::with_capacity(hidden_chars + last_4_chars_len);

    unsafe {
        for _ in 0..hidden_chars {
            res.push_unchecked(b'#');
        }
        res.extend_from_slice_unchecked(cc.as_bytes().get_unchecked(cc.len() - last_4_chars_len..));
    }

    unsafe { String::from_utf8_unchecked(res) }
}
