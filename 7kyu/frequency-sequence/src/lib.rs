//! <https://www.codewars.com/kata/585a033e3a36cdc50a00011c/train/rust>

use core::{hash::BuildHasherDefault, hint::unreachable_unchecked};
use digital::{MaxLenBase10, WriteNumUnchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use unchecked_std::prelude::*;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn freq_seq(s: &str, sep: &str) -> String {
    let cap = s.len();
    let mut counts = FxHashMap::with_capacity_and_hasher(cap, Default::default());
    for c in s.chars() {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
        }
        counts
            .entry(c)
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
            res.write_num_unchecked(*counts.get(&c).unwrap(), 10, false, false);
        }
    }

    res
}
