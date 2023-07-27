//! <https://www.codewars.com/kata/5af15a37de4c7f223e00012d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use vqsort::VqSort;

pub fn men_from_boys(xs: &[i16]) -> Vec<i16> {
    let mut res = xs.to_vec();
    res.sort_unstable_by_key(|x| x % 2 != 0);
    let even_count = res.partition_point(|x| x % 2 == 0);
    VqSort::sort_ascending(&mut res[..even_count]);
    VqSort::sort_descending(&mut res[even_count..]);
    res.dedup();
    res
}
