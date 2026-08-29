//! <https://www.codewars.com/kata/52de553ebb55d1fca3000371/train/rust>

#![feature(portable_simd)]

use std::{hint::unreachable_unchecked, simd::prelude::*};

const LANES: usize = 16;

pub fn find_missing(seq: &[i32]) -> i32 {
    let first = *seq.first().unwrap();
    let last = *seq.last().unwrap();
    let step = ((last as isize - first as isize) / seq.len() as isize) as i32;
    if step == 0 {
        return first;
    }

    let step_vec = Simd::<i32, LANES>::splat(step);
    let chunks = (seq.len() - 1) / LANES;
    for i in 0..chunks {
        let base = i * LANES;
        let (left_slice, right_slice) = (unsafe { seq.get_unchecked(base..) }, unsafe {
            seq.get_unchecked(base + 1..)
        });
        if left_slice.len() < LANES || right_slice.len() < LANES {
            unsafe { unreachable_unchecked() };
        }
        let left = Simd::<i32, LANES>::from_slice(left_slice);
        let right = Simd::<i32, LANES>::from_slice(right_slice);
        let diffs = right - left;
        let mask = diffs.simd_ne(step_vec);
        if mask.any() {
            let lane = mask.to_bitmask().trailing_zeros() as usize;
            return unsafe { seq.get_unchecked(base + lane) } + step;
        }
    }

    let tail_start = chunks * LANES;
    for pair in unsafe { seq.get_unchecked(tail_start..) }.windows(2) {
        if pair[1] - pair[0] != step {
            return pair[0] + step;
        }
    }

    0
}
