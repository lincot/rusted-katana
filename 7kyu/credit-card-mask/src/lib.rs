//! <https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust>

use core::iter::repeat;
use my_prelude::prelude::*;

pub fn maskify(cc: &str) -> String {
    let chars: Vec<_> = cc.chars().collect();

    let hidden_chars = chars.len().saturating_sub(4);
    let last_4_chars_len: usize = chars.iter().rev().take(4).map(|c| c.len_utf8()).sum();

    let mut res = Vec::with_capacity(hidden_chars + last_4_chars_len);

    res.extend(repeat(b'#').take(hidden_chars));
    unsafe {
        res.extend_from_slice_unchecked(cc.as_bytes().get_unchecked(cc.len() - last_4_chars_len..));
    }

    unsafe { String::from_utf8_unchecked(res) }
}
