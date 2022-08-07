//! <https://www.codewars.com/kata/5b39e3772ae7545f650000fc/train/rust>

use my_prelude::prelude::*;
use rustc_hash::FxHashSet;

pub fn remove_duplicate_words(s: &str) -> String {
    // arbitrary capacity
    let cap = s.len() / 4;
    let mut words_set = FxHashSet::with_capacity_and_hasher(cap, Default::default());

    let mut unique_words = s.split_ascii_whitespace().filter(|&w| words_set.insert(w));

    let mut res = String::with_capacity(s.len());

    if let Some(first) = unique_words.next() {
        unsafe { res.push_str_unchecked(first) };
    }
    for word in unique_words {
        unsafe {
            res.push_unchecked(' ');
            res.push_str_unchecked(word);
        }
    }

    res
}
