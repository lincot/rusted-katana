//! <https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust>

use my_prelude::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut counts = FxHashMap::with_capacity_and_hasher(sip.len(), Default::default());
    let mut order = Vec::with_capacity(sip.len());

    for c in sip.chars() {
        if counts.len() == counts.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        match counts.entry(c) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                unsafe { order.push_unchecked(c) };
                e.insert(1);
            }
        }
    }

    order.into_iter().map(|c| (c, counts[&c])).collect()
}
