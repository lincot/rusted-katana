//! <https://www.codewars.com/kata/546f922b54af40e1e90001da/train/rust>

use digital::WriteNumUnchecked;
use unchecked_std::prelude::*;

pub fn alphabet_position(text: &str) -> String {
    let mut res = Vec::with_capacity(text.len().checked_mul(3).unwrap());
    for b in text.bytes() {
        let num = if b.is_ascii_lowercase() {
            b - b'a' + 1
        } else if b.is_ascii_uppercase() {
            b - b'A' + 1
        } else {
            continue;
        };
        unsafe {
            res.write_num_unchecked(num, 10, false, false);
            res.push_unchecked(b' ');
        }
    }
    res.pop();
    unsafe { String::from_utf8_unchecked(res) }
}
