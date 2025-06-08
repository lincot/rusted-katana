//! <https://www.codewars.com/kata/5ce399e0047a45001c853c2b/train/rust>

use unchecked_std::prelude::*;

pub fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut sum: u64 = ls.iter().sum();
    let mut res = Vec::with_capacity(ls.len() + 1);
    unsafe { res.push_unchecked(sum) };
    for &x in ls {
        sum -= x;
        unsafe { res.push_unchecked(sum) };
    }
    res
}
