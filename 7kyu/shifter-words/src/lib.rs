//! <https://www.codewars.com/kata/603b2bb1c7646d000f900083/train/rust>

use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn shifter(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut set = FxHashMap::with_capacity_and_hasher(s.len() / 2 + 1, Default::default());

    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|bytes| unsafe { core::str::from_utf8_unchecked(bytes) })
        .filter(|word| word.bytes().all(|b| b"HINOSXZMW".contains(&b)))
    {
        if set.len() == set.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = set.entry(word) {
            e.insert(());
        }
    }

    set.len()
}
