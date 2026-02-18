//! <https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/rust>

use core::hint::unreachable_unchecked;
use std::collections::HashMap;

pub fn count(input: &str) -> HashMap<char, i32> {
    let mut res = HashMap::with_capacity(input.len());
    for ch in input.chars() {
        if res.len() == res.capacity() {
            unsafe { unreachable_unchecked() };
        }
        res.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
    res
}
