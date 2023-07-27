//! <https://www.codewars.com/kata/55eea63119278d571d00006a/train/rust>

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use vqsort::VqSort;

pub fn next_id(ids: &[usize]) -> usize {
    let mut ids: Box<[_]> = ids.into();
    VqSort::sort_ascending(&mut ids);
    let mut res = 0;
    for &i in &*ids {
        if i > res {
            return res;
        }
        res = i + 1;
    }
    res
}
