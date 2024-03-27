//! <https://www.codewars.com/kata/5a262cfb8f27f217f700000b/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;
use unchecked_core::ExtendUnchecked;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn solve(a: &str, b: &str) -> String {
    let mut map_a = FxHashMap::with_capacity_and_hasher(a.len(), Default::default());
    for c in a.chars() {
        if map_a.len() == map_a.capacity() {
            unsafe { unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = map_a.entry(c) {
            e.insert(());
        }
    }
    let mut map_b = FxHashMap::with_capacity_and_hasher(b.len(), Default::default());
    for c in b.chars() {
        if map_b.len() == map_b.capacity() {
            unsafe { unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = map_b.entry(c) {
            e.insert(());
        }
    }

    let mut res = String::with_capacity(a.len() + b.len());
    unsafe {
        res.extend_unchecked(a.chars().filter(|c| !map_b.contains_key(c)));
        res.extend_unchecked(b.chars().filter(|c| !map_a.contains_key(c)));
    }
    res
}
