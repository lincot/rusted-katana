//! <https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust>

use unchecked_core::PushUnchecked;

pub fn maps(values: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(values.len());
    for &x in values {
        unsafe { res.push_unchecked(x) };
    }
    if res.len() != values.len() {
        unsafe { core::hint::unreachable_unchecked() };
    }
    res
}
