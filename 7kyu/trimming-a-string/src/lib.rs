//! <https://www.codewars.com/kata/563fb342f47611dae800003c/train/rust>

use unchecked_std::prelude::*;

pub fn trim(phrase: &str, length: usize) -> String {
    if phrase.len() <= length {
        return phrase.into();
    }
    if phrase.is_ascii() {
        return unsafe { String::from_utf8_unchecked(trim_bytes(phrase.as_bytes(), length)) };
    }

    phrase.char_indices().nth(length).map_or_else(
        || phrase.into(),
        |(ci, _)| {
            let mut res = String::with_capacity(ci + 3);
            unsafe {
                res.push_str_unchecked(phrase.get_unchecked(..ci));
                res.push_str_unchecked("...");
            }
            res
        },
    )
}

unsafe fn trim_bytes(phrase: &[u8], length: usize) -> Vec<u8> {
    let length = if length > 3 { length - 3 } else { length };
    let mut res = Vec::with_capacity(length + 3);
    unsafe {
        res.extend_from_slice_unchecked(phrase.get_unchecked(..length));
        res.extend_from_slice_unchecked(b"...");
    }
    res
}
