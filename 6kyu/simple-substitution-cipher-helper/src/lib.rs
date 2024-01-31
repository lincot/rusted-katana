//! <https://www.codewars.com/kata/52eb114b2d55f0e69800078d/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use rustc_hash::FxHasher;
use unchecked::PushUnchecked;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub struct Cipher {
    encoder: FxHashMap<char, char>,
    decoder: FxHashMap<char, char>,
}

impl Cipher {
    pub fn new(map1: &str, map2: &str) -> Self {
        let mut encoder = FxHashMap::with_capacity_and_hasher(map1.len(), Default::default());
        let mut decoder = FxHashMap::with_capacity_and_hasher(map1.len(), Default::default());

        for (c1, c2) in map1.chars().zip(map2.chars()) {
            if encoder.len() == encoder.capacity() {
                unsafe { unreachable_unchecked() };
            }
            if let Entry::Vacant(e) = encoder.entry(c1) {
                e.insert(c2);
            }

            if decoder.len() == decoder.capacity() {
                unsafe { unreachable_unchecked() };
            }
            if let Entry::Vacant(e) = decoder.entry(c2) {
                e.insert(c1);
            }
        }

        Self { encoder, decoder }
    }

    pub fn encode(&self, string: &str) -> String {
        let mut res = String::with_capacity(4 * string.len());
        for c in string.chars() {
            unsafe { res.push_unchecked(*self.encoder.get(&c).unwrap_or(&c)) };
        }
        res
    }

    pub fn decode(&self, string: &str) -> String {
        let mut res = String::with_capacity(4 * string.len());
        for c in string.chars() {
            unsafe { res.push_unchecked(*self.decoder.get(&c).unwrap_or(&c)) };
        }
        res
    }
}
