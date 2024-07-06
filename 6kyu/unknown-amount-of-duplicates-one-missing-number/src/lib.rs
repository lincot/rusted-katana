//! <https://www.codewars.com/kata/5a5cdb07fd56cbdd3c00005b/train/rust>

use unchecked_std::prelude::*;

pub fn find_dups_miss(arr: &[u32]) -> (u32, Vec<u32>) {
    let (a, b) = min_max(arr.iter().copied()).unwrap();

    let mut counts = vec![0; (b - a + 1) as usize];
    for x in arr {
        unsafe { *counts.get_unchecked_mut((x - a) as usize) += 1 };
    }

    let mut res = (0, Vec::with_capacity(counts.len()));
    for (n, count) in (a..).zip(counts) {
        if count == 0 {
            res.0 = n;
        } else if count != 1 {
            unsafe { res.1.push_unchecked(n) };
        }
    }
    res
}

fn min_max<It: Iterator<Item = Item>, Item: Ord + Copy>(mut it: It) -> Option<(Item, Item)> {
    let first = it.next()?;
    Some(it.fold((first, first), |(min, max), new| {
        (min.min(new), max.max(new))
    }))
}
