//! <https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust>

use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut counts = FxHashMap::with_capacity_and_hasher(sip.len(), Default::default());
    let mut res = Vec::<(char, i32)>::with_capacity(sip.len());

    for c in sip.chars() {
        if counts.len() == counts.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        match counts.entry(c) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                unsafe {
                    let len = res.len();
                    res.get_unchecked_mut(len).0 = c;
                    res.set_len(len + 1);
                }
                e.insert(1);
            }
        }
    }

    for x in &mut res {
        x.1 = unsafe { *counts.get(&x.0).unwrap_unchecked() };
    }

    res
}
