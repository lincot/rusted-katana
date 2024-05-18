//! <https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust>

use unchecked_std::prelude::*;

pub fn reverse_words(words: &str) -> String {
    let words = words.as_bytes();
    let mut res = Vec::with_capacity(words.len());

    let mut last_space = words.len();
    for (i, &b) in words.iter().enumerate().rev() {
        if b == b' ' {
            unsafe {
                res.extend_from_slice_unchecked(words.get_unchecked(i + 1..last_space));
                res.push_unchecked(b' ');
            }
            last_space = i;
        }
    }
    unsafe { res.extend_from_slice_unchecked(words.get_unchecked(..last_space)) };

    unsafe { String::from_utf8_unchecked(res) }
}
