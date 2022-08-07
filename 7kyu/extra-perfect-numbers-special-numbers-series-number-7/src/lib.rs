//! <https://www.codewars.com/kata/5a662a02e626c54e87000123/train/rust>

use my_prelude::prelude::*;

pub fn extra_perfect(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(((n + 1) / 2) as usize);
    let mut i = 1;
    while i <= n {
        unsafe { res.push_unchecked(i) };
        i += 2;
    }
    res
}
