//! <https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust>

use my_prelude::prelude::*;

pub fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let cap = (limit / n) as usize;
    let mut res = Vec::with_capacity(cap);

    let mut i = n;
    while i <= limit {
        unsafe {
            res.push_unchecked(i);
        }
        i += n;
    }

    res
}
