//! <https://www.codewars.com/kata/5af15a37de4c7f223e00012d/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::cmp::Reverse;
use my_prelude::prelude::*;

pub fn men_from_boys(xs: &[i16]) -> Vec<i16> {
    let mut boys = Vec::with_capacity(xs.len());
    let mut men = Vec::with_capacity(xs.len());

    for &x in xs {
        if x % 2 == 0 {
            unsafe { boys.push_unchecked(x) };
        } else {
            unsafe { men.push_unchecked(x) };
        }
    }

    boys.sort_unstable();
    boys.dedup();

    men.sort_unstable_by_key(|&v| Reverse(v));
    men.dedup();

    unsafe { boys.extend_from_slice_unchecked(&men) };

    boys
}
