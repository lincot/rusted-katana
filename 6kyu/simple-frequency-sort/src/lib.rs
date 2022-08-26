//! <https://www.codewars.com/kata/5a8d2bf60025e9163c0000bc/train/rust>

use core::cmp::Reverse;
use my_prelude::prelude::*;
use rustc_hash::FxHashMap;

pub fn solve(vec: &[i32]) -> Vec<i32> {
    let mut counts = FxHashMap::with_capacity_and_hasher(vec.len(), Default::default());

    for &x in vec {
        *counts.entry(x).or_insert(0usize) += 1;
    }

    let mut counts: Vec<_> = counts.into_iter().collect();

    counts.sort_unstable_by_key(|&(x, _)| x);
    counts.sort_by_key(|&(_, c)| Reverse(c));

    let mut res = Vec::with_capacity(vec.len());

    for (x, c) in counts {
        for _ in 0..c {
            unsafe { res.push_unchecked(x) };
        }
    }

    res
}
