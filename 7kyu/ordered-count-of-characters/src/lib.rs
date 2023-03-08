//! <https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut counts = FxHashMap::with_capacity_and_hasher(sip.len(), Default::default());
    let mut res = Vec::<(char, i32)>::with_capacity(sip.len());

    for c in sip.chars() {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
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
