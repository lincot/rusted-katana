//! <https://www.codewars.com/kata/60490a215465720017ab58fa/train/rust>

#![no_std]

use core::{
    hash::{BuildHasherDefault, Hasher},
    hint::unreachable_unchecked,
};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;

#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!();
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

type IdHashMap<K, V> = HashMap<K, V, BuildHasherDefault<IdHasher>>;

pub fn gangs(divisors: &[u32], k: u32) -> u32 {
    let mut map = IdHashMap::with_capacity_and_hasher(k as _, Default::default());
    for n in 1..=k {
        let mut h = FxHasher::default();
        for &divisor in divisors {
            if n % divisor == 0 {
                h.write_u32(divisor);
            }
        }
        if map.len() == map.capacity() {
            unsafe { unreachable_unchecked() };
        }
        if let Entry::Vacant(e) = map.entry(h.finish()) {
            e.insert(());
        }
    }
    map.len() as _
}
