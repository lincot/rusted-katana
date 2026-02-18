//! <https://www.codewars.com/kata/51e0007c1f9378fa810002a9/train/rust>

use unchecked_std::prelude::*;

pub fn parse(code: &str) -> Vec<i32> {
    let mut val = 0;
    let mut res = Vec::with_capacity(code.len());
    for b in code.bytes() {
        match b {
            b'i' => val += 1,
            b'd' => val -= 1,
            b's' => val *= val,
            b'o' => unsafe { res.push_unchecked(val) },
            _ => {}
        }
    }
    res
}
