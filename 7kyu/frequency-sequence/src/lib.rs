//! <https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use digital::{MaxLenBase10, WriteNumUnchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use unchecked_std::prelude::*;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn freq_seq(s: &str, sep: &str) -> String {
    if s.is_ascii() {
        return freq_seq_bytes(s.as_bytes(), sep);
    }

    let cap = s.len();
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    for ch in s.chars() {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
        }
        counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1usize);
    }

    let cap = (usize::MAX_LEN_BASE10 + sep.len()) * s.len();
    let mut res = String::with_capacity(cap);

    for (i, c) in s.chars().enumerate() {
        unsafe {
            if i != 0 {
                res.push_str_unchecked(sep);
            }
            let count = *counts.get(&c).unwrap();
            res.write_num_unchecked(count, 10, false, false);
        }
    }

    res
}

fn freq_seq_bytes(s: &[u8], sep: &str) -> String {
    let mut counts = [0usize; 256];

    for &b in s {
        counts[b as usize] += 1;
    }

    let cap = (usize::MAX_LEN_BASE10 + sep.len()) * s.len();
    let mut res = String::with_capacity(cap);

    for (i, &b) in s.iter().enumerate() {
        unsafe {
            if i != 0 {
                res.push_str_unchecked(sep);
            }
            let count = counts[b as usize];
            res.write_num_unchecked(count, 10, false, false);
        }
    }

    res
}
