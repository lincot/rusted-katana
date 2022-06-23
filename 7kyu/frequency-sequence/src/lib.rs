//! <https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust>

use rustc_hash::FxHashMap;

pub fn freq_seq(s: &str, sep: &str) -> String {
    // worst case capacity
    let cap = s.len();
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    for c in s.chars() {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    // 2 digits and sep per char
    let cap = (2 + sep.len()) * counts.len();
    let mut res = String::with_capacity(cap);

    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        res.push_str(&counts.get(&c).unwrap().to_string());
    }
    for c in chars {
        res.push_str(sep);
        res.push_str(&counts.get(&c).unwrap().to_string());
    }

    res
}
