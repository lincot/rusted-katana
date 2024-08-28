//! <https://www.codewars.com/kata/5848565e273af816fb000449/train/rust>

use digital::WriteNumUnchecked;
use unchecked_std::prelude::*;

pub fn encrypt_this(text: &str) -> String {
    let mut res = String::with_capacity(8 * text.len() / 3 + 2);
    for word in text.as_bytes().split(|&b| b == b' ') {
        let word = unsafe { core::str::from_utf8_unchecked(word) };
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        unsafe {
            res.write_num_unchecked(first as u8, 10, false, false);
            let Some(second) = chars.next() else {
                res.push_unchecked(' ');
                continue;
            };
            let Some(last) = chars.next_back() else {
                res.push_unchecked(second);
                res.push_unchecked(' ');
                continue;
            };
            res.push_unchecked(last);
            res.push_str_unchecked(
                word.get_unchecked(
                    first.len_utf8() + second.len_utf8()..word.len() - last.len_utf8(),
                ),
            );
            res.push_unchecked(second);
            res.push_unchecked(' ');
        }
    }
    unsafe { res.as_mut_vec() }.pop();
    res
}
