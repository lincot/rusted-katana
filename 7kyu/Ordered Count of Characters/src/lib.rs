//! <https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust>

use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    // arbitrary capacity
    let cap = 64.min(sip.len());
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    let mut order = Vec::with_capacity(cap);

    for c in sip.chars() {
        match counts.entry(c) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                order.push(c);
                e.insert(1);
            }
        }
    }

    order.into_iter().map(|c| (c, counts[&c])).collect()
}
