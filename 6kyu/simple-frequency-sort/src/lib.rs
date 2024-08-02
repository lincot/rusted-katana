//! <https://www.codewars.com/kata/5a8d2bf60025e9163c0000bc/train/rust>

use core::{cmp::Reverse, hash::BuildHasherDefault, hint::unreachable_unchecked};
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use unchecked_std::prelude::*;

type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn solve(vec: &[i32]) -> Vec<i32> {
    let mut counts = FxHashMap::with_capacity_and_hasher(vec.len(), Default::default());
    for &x in vec {
        if counts.len() == counts.capacity() {
            unsafe { unreachable_unchecked() };
        }
        counts
            .entry(x)
            .and_modify(|count| *count += 1)
            .or_insert(1usize);
    }

    let mut counts_arr = Vec::with_capacity(counts.len());
    unsafe { counts_arr.set_len(counts.len()) };
    let mut counts_arr = counts_arr.into_boxed_slice();
    for (c, x) in counts_arr.iter_mut().zip(counts) {
        *c = x;
    }
    if counts_arr.len() < 200 {
        counts_arr.sort_unstable_by_key(|&(x, _)| x);
    } else {
        radsort::sort_by_key(&mut counts_arr, |&(x, _)| x);
    }
    if counts_arr.len() < 64 {
        counts_arr.sort_by_key(|&(_, c)| Reverse(c));
    } else {
        radsort::sort_by_key(&mut counts_arr, |&(_, c)| usize::MAX - c);
    }

    let mut res = Vec::with_capacity(vec.len());
    for &(x, c) in &*counts_arr {
        for _ in 0..c {
            unsafe { res.push_unchecked(x) };
        }
    }
    res
}
