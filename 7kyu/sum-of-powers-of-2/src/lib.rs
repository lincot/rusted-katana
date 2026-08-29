//! <https://www.codewars.com/kata/5d9f95424a336600278a9632/train/rust>

use unchecked_std::prelude::*;

pub fn powers(mut n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(u32::BITS as _);
    while n != 0 {
        unsafe { res.push_unchecked(1 << n.trailing_zeros()) };
        n &= n - 1;
    }
    res
}
