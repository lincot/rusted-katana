//! <https://www.codewars.com/kata/5b39e3772ae7545f650000fc/train/rust>

use rustc_hash::FxHashSet;

pub fn remove_duplicate_words(s: &str) -> String {
    // arbitrary capacity
    let cap = s.len() / 4;
    let mut words_set = FxHashSet::with_capacity_and_hasher(cap, Default::default());

    let mut unique_words = s.split_ascii_whitespace().filter(|&w| words_set.insert(w));

    // worst case capacity
    let cap = s.len();
    let mut res = String::with_capacity(cap);

    res.push_str(if let Some(first) = unique_words.next() {
        first
    } else {
        return res;
    });

    for word in unique_words {
        res.push(' ');
        res.push_str(word);
    }

    res
}
