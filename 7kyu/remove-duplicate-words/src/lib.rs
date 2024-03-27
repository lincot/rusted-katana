//! <https://www.codewars.com/kata/5b39e3772ae7545f650000fc/train/rust>

use unchecked_core::{PushStrUnchecked, PushUnchecked};

pub fn remove_duplicate_words(s: &str) -> String {
    let mut words = Vec::with_capacity(s.len() / 2 + 1);
    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|bytes| unsafe { core::str::from_utf8_unchecked(bytes) })
    {
        if !words.contains(&word) {
            unsafe { words.push_unchecked(word) };
        }
    }

    let mut res = String::with_capacity(s.len());
    let mut words = words.into_iter();
    if let Some(word) = words.next() {
        unsafe { res.push_str_unchecked(word) };
    }
    for word in words {
        unsafe {
            res.push_unchecked(' ');
            res.push_str_unchecked(word);
        }
    }
    res
}
