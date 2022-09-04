//! <https://www.codewars.com/kata/5a8d2bf60025e9163c0000bc/train/rust>

use core::cmp::Reverse;
use my_prelude::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

pub fn solve(vec: &[i32]) -> Vec<i32> {
    let mut counts = FxHashMap::with_capacity_and_hasher(vec.len(), Default::default());
    for &x in vec {
        if counts.len() == counts.capacity() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        match counts.entry(x) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    let mut counts_array = Vec::with_capacity(counts.len());
    unsafe { counts_array.set_len(counts.len()) };
    let mut counts_arr = counts_array.into_boxed_slice();
    let mut counts_vec_ptr = counts_arr.as_mut_ptr();
    for x in counts {
        unsafe {
            *counts_vec_ptr = x;
            counts_vec_ptr = counts_vec_ptr.add(1);
        }
    }
    counts_arr.sort_unstable_by_key(|&(x, _)| x);
    counts_arr.sort_by_key(|&(_, c)| Reverse(c));

    let mut res = Vec::with_capacity(vec.len());
    for &(x, c) in counts_arr.iter() {
        for _ in 0..c {
            unsafe { res.push_unchecked(x) };
        }
    }
    res
}
