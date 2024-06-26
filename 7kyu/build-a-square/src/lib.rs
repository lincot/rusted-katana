//! <https://www.codewars.com/kata/59a96d71dbe3b06c0200009c/train/rust>

use unchecked_std::prelude::*;

pub fn generate_shape(n: i32) -> String {
    let n = n as usize;
    let mut res = Vec::with_capacity(n * n + n);
    for i in 0..n {
        if i != 0 {
            unsafe { res.push_unchecked(b'\n') };
        }
        for _ in 0..n {
            unsafe { res.push_unchecked(b'+') };
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
