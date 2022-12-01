//! <https://www.codewars.com/kata/5663f5305102699bad000056/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    let Some((a1_min, a1_max)) = min_max(a1.into_iter().map(str::len)) else {
         return -1;
    };
    let Some((a2_min, a2_max)) = min_max(a2.into_iter().map(str::len)) else {
         return -1;
    };
    (a1_max - a2_min).max(a2_max - a1_min) as _
}

fn min_max<It: Iterator<Item = Item>, Item: Ord + Copy>(mut it: It) -> Option<(Item, Item)> {
    let first = it.next()?;

    Some(it.fold((first, first), |(min, max), new| {
        (min.min(new), max.max(new))
    }))
}
