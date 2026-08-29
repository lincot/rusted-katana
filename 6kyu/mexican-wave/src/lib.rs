//! <https://www.codewars.com/kata/58f5c63f1e26ecda7e000029/train/rust>

use core::mem::transmute;

use unchecked_std::prelude::*;

pub fn wave(s: &str) -> Vec<String> {
    if s.is_ascii() {
        #[expect(clippy::transmute_undefined_repr)]
        return unsafe { transmute::<Vec<Vec<u8>>, Vec<String>>(wave_bytes(s.as_bytes())) };
    }

    let mut res = Vec::with_capacity(s.len());

    for (i, ch) in s.char_indices() {
        if ch.is_whitespace() {
            continue;
        }

        let mut new_s = String::with_capacity(s.len() + 4);
        unsafe {
            new_s.push_str_unchecked(s.get_unchecked(..i));
            new_s.extend_unchecked(ch.to_uppercase());
            new_s.push_str_unchecked(s.get_unchecked(i + ch.len_utf8()..));
        }
        unsafe { res.push_unchecked(new_s) };
    }

    res
}

pub fn wave_bytes(s: &[u8]) -> Vec<Vec<u8>> {
    let mut res = Vec::with_capacity(s.len());

    for (i, b) in s.iter().enumerate() {
        if b.is_ascii_whitespace() {
            continue;
        }

        let mut new_s = Vec::with_capacity(s.len());
        unsafe {
            new_s.extend_from_slice(s.get_unchecked(..i));
            new_s.push_unchecked(b.to_ascii_uppercase());
            new_s.extend_from_slice(s.get_unchecked(i + 1..));
        }
        unsafe { res.push_unchecked(new_s) };
    }

    res
}
