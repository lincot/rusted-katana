//! <https://www.codewars.com/kata/52eb114b2d55f0e69800078d/train/rust>

use rustc_hash::FxHashMap;

pub struct Cipher {
    encoder: FxHashMap<char, char>,
    decoder: FxHashMap<char, char>,
}

impl Cipher {
    pub fn new(map1: &str, map2: &str) -> Self {
        let mut encoder = FxHashMap::with_capacity_and_hasher(map1.len(), Default::default());
        let mut decoder = FxHashMap::with_capacity_and_hasher(map1.len(), Default::default());

        for (c1, c2) in map1.chars().zip(map2.chars()) {
            encoder.insert(c1, c2);
            decoder.insert(c2, c1);
        }

        Self { encoder, decoder }
    }

    pub fn encode(&self, string: &str) -> String {
        let mut res = String::with_capacity(string.len());
        for c in string.chars() {
            res.push(*self.encoder.get(&c).unwrap_or(&c));
        }
        res
    }

    pub fn decode(&self, string: &str) -> String {
        let mut res = String::with_capacity(string.len());
        for c in string.chars() {
            res.push(*self.decoder.get(&c).unwrap_or(&c));
        }
        res
    }
}
