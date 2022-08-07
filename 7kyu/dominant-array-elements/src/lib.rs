//! <https://www.codewars.com/kata/5a04133e32b8b998dc000089/train/rust>

use my_prelude::prelude::*;

pub fn solve(arr: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(arr.len());
    let mut max = 0;

    for &x in arr.iter().rev() {
        if x > max {
            unsafe { res.push_unchecked(x) };
            max = x;
        }
    }

    res.reverse();

    res
}
