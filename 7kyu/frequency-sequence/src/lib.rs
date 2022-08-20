//! <https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust>

use my_prelude::prelude::*;
use rustc_hash::FxHashMap;

pub fn freq_seq(s: &str, sep: &str) -> String {
    // worst case capacity
    let cap = s.len();
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    for c in s.chars() {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    let cap = (20 + sep.len()) * s.len();
    let mut res = String::with_capacity(cap);

    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        unsafe {
            res.write_num_unchecked(*counts.get(&c).unwrap());
        }
    }
    for c in chars {
        unsafe {
            res.push_str_unchecked(sep);
            res.write_num_unchecked(*counts.get(&c).unwrap());
        }
    }

    res
}
