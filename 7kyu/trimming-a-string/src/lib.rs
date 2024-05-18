//! <https://www.codewars.com/kata/563fb342f47611dae800003c/train/rust>

use unchecked_std::prelude::*;

pub fn trim(phrase: &str, length: usize) -> String {
    if phrase.len() <= length {
        return phrase.into();
    }
    let length = if length > 3 { length - 3 } else { length };
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
