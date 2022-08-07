//! <https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust>

use my_prelude::prelude::*;

pub fn name_shuffler(s: &str) -> String {
    let s = s.as_bytes();
    let mut res = Vec::with_capacity(s.len());

    let mut first_len = 0;
    for &b in s {
        if b == b' ' {
            break;
        }
        first_len += 1;
    }

    unsafe {
        res.extend_from_slice_unchecked(s.get_unchecked(first_len + 1..));
        res.push_unchecked(b' ');
        res.extend_from_slice_unchecked(s.get_unchecked(..first_len));
    }

    unsafe { String::from_utf8_unchecked(res) }
}
