//! <https://www.codewars.com/kata/5a262cfb8f27f217f700000b/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashSet;
use rustc_hash::FxHasher;
use unchecked_std::prelude::*;

type FxHashSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;

pub fn solve(a: &str, b: &str) -> String {
    let mut map_a = FxHashSet::with_capacity_and_hasher(a.len(), Default::default());
    for c in a.chars() {
        if map_a.len() == map_a.capacity() {
            unsafe { unreachable_unchecked() };
        }
        map_a.insert(c);
    }
    let mut map_b = FxHashSet::with_capacity_and_hasher(b.len(), Default::default());
    for c in b.chars() {
        if map_b.len() == map_b.capacity() {
            unsafe { unreachable_unchecked() };
        }
        map_b.insert(c);
    }

    let mut res = String::with_capacity(a.len() + b.len());
    unsafe {
        res.extend_unchecked(a.chars().filter(|c| !map_b.contains(c)));
        res.extend_unchecked(b.chars().filter(|c| !map_a.contains(c)));
    }
    res
}
