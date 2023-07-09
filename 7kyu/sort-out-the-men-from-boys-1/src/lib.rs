//! <https://www.codewars.com/kata/5af15a37de4c7f223e00012d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::cmp::Reverse;

pub fn men_from_boys(xs: &[i16]) -> Vec<i16> {
    let mut res = xs.to_vec();
    res.sort_unstable_by_key(|x| x % 2 != 0);
    let even_count = res.partition_point(|x| x % 2 == 0);

    let boys = &mut res[..even_count];
    if boys.len() < 80 {
        boys.sort_unstable();
    } else {
        radsort::sort(boys);
    }

    let men = &mut res[even_count..];
    if men.len() < 80 {
        men.sort_unstable_by_key(|&v| Reverse(v));
    } else {
        radsort::sort_by_key(men, |x| i16::MAX - x);
    }

    res.dedup();
    res
}
