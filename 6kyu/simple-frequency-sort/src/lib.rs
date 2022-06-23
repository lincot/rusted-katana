//! <https://www.codewars.com/kata/5a8d2bf60025e9163c0000bc/train/rust>

use rustc_hash::FxHashMap;
use std::iter::repeat;

pub fn solve(vec: &[i32]) -> Vec<i32> {
    let mut counts = FxHashMap::with_capacity_and_hasher(vec.len(), Default::default());

    for &x in vec {
        *counts.entry(x).or_insert(0usize) += 1;
    }

    let mut counts: Vec<_> = counts.iter().collect();

    counts.sort_unstable_by_key(|&(x, _)| x);
    counts.sort_by_key(|&(_, c)| std::cmp::Reverse(c));

    let mut res = Vec::with_capacity(vec.len());

    for (x, &c) in counts {
        res.extend(repeat(x).take(c));
    }

    res
}
