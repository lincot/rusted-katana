//! <https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::{hash_map::Entry, HashMap};
use prelude::*;
use rustc_hash::FxHasher;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn freq_seq(s: &str, sep: &str) -> String {
    let cap = s.len();
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    for c in s.chars() {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
        }
        match counts.entry(c) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1usize;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    let cap = (USIZE_MAX_LEN + sep.len()) * s.len();
    let mut res = String::with_capacity(cap);

    for (i, c) in s.chars().enumerate() {
        unsafe {
            if i != 0 {
                res.push_str_unchecked(sep);
            }
            res.write_num_unchecked(*counts.get(&c).unwrap(), false, false);
        }
    }

    res
}
