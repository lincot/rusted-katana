//! <https://www.codewars.com/kata/55eea63119278d571d00006a/train/rust>

#![no_std]

extern crate alloc;
use alloc::boxed::Box;

pub fn next_id(ids: &[usize]) -> usize {
    let mut ids: Box<[_]> = ids.into();
    if ids.len() < 10000 {
        ids.sort_unstable();
    } else {
        radsort::sort(&mut ids);
    }
    let mut res = 0;
    for &i in &*ids {
        if i > res {
            return res;
        }
        res = i + 1;
    }
    res
}
