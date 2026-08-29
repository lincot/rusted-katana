//! <https://www.codewars.com/kata/585d7d5adb20cf33cb000235/train/rust>

#![feature(portable_simd)]

use std::simd::prelude::*;

const LANES: usize = 16;

pub fn find_uniq(arr: &[f64]) -> f64 {
    let &[first, second, third, ..] = arr else {
        panic!();
    };
    if first != second {
        return if first == third { second } else { first };
    }

    let chunks = arr[2..].as_chunks::<{ LANES }>();
    for chunk in chunks.0 {
        let mask = Simd::<f64, LANES>::from_slice(chunk)
            .simd_ne(Simd::splat(first))
            .to_bitmask();
        if mask != 0 {
            return chunk[mask.trailing_zeros() as usize];
        }
    }
    chunks.1.iter().copied().find(|&x| x != first).unwrap()
}
