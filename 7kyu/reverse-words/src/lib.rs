//! <https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust>

use unchecked_core::{ExtendUnchecked, PushUnchecked};

pub fn reverse_words(str: &str) -> String {
    let mut res = String::with_capacity(str.len());

    for (i, w) in str
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|bytes| {
            unsafe { core::str::from_utf8_unchecked(bytes) }
                .chars()
                .rev()
        })
        .enumerate()
    {
        if i != 0 {
            unsafe { res.push_unchecked(' ') };
        }
        unsafe { res.extend_unchecked(w) };
    }

    res
}
