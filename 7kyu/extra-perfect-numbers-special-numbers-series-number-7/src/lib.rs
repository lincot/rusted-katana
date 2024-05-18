//! <https://www.codewars.com/kata/5a662a02e626c54e87000123/train/rust>

use unchecked_std::prelude::*;

pub fn extra_perfect(n: u32) -> Vec<u32> {
    let cap = (n + 1) / 2;
    let mut res = Vec::with_capacity(cap as _);
    let mut i = 1;
    for _ in 0..cap {
        unsafe { res.push_unchecked(i) };
        i += 2;
    }
    res
}
