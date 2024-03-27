//! <https://www.codewars.com/kata/57f8ee485cae443c4d000127/train/rust>

use unchecked_core::PushUnchecked;

pub fn spacify(s: &str) -> String {
    let mut res = String::with_capacity(2 * s.len());
    let mut s = s.chars();
    if let Some(ch) = s.next() {
        unsafe { res.push_unchecked(ch) };
    }
    for ch in s {
        unsafe {
            res.push_unchecked(' ');
            res.push_unchecked(ch);
        }
    }
    res
}
