//! <https://www.codewars.com/kata/60490a215465720017ab58fa/train/rust>

#![no_std]

use core::hash::{BuildHasherDefault, Hash, Hasher};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn gangs(divisors: &[u32], k: u32) -> u32 {
    let mut map = FxHashMap::with_capacity_and_hasher(k as _, Default::default());
    for n in 1..=k {
        let mut h = FxHasher::default();
        for divisor in divisors {
            if n % divisor == 0 {
                divisor.hash(&mut h);
            }
        }
        if map.len() == map.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = map.entry(h.finish()) {
            e.insert(());
        }
    }
    map.len() as _
}
