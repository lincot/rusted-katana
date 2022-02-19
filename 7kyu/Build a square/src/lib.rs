//! <https://www.codewars.com/kata/59a96d71dbe3b06c0200009c/train/rust>

use std::iter::repeat;

pub fn generate_shape(n: i32) -> String {
    assert!(n > 0);
    let n = n as usize;
    let mut res = Vec::with_capacity(n * n + n - 1);

    res.extend(repeat(b'+').take(n));
    for _ in 1..n {
        res.push(b'\n');
        res.extend(repeat(b'+').take(n));
    }

    unsafe { String::from_utf8_unchecked(res) }
}
