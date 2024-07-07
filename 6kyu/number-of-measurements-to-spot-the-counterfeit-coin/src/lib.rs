//! <https://www.codewars.com/kata/59530d2401d6039f8600001f/train/rust>

#![feature(portable_simd)]

use core::simd::Simd;
use std::simd::cmp::SimdPartialOrd;

pub fn how_many_measurements(n: u64) -> u32 {
    const POWERS_OF_3: [u64; 41] = {
        let mut res = [1; 41];
        let mut i = 1;
        while i < res.len() {
            res[i] = 3 * res[i - 1];
            i += 1;
        }
        res
    };

    let mut i = 0;
    let n_simd = Simd::splat(n);

    while i < 40 {
        let powers_simd: Simd<u64, 8> = Simd::from_slice(&POWERS_OF_3[i..i + 8]);
        let matches_mask = n_simd.simd_le(powers_simd);
        if matches_mask.any() {
            let which_match = matches_mask.to_bitmask().trailing_zeros();
            return i as u32 + which_match;
        }
        i += 8;
    }

    if n <= *POWERS_OF_3.last().unwrap() {
        40
    } else {
        41
    }
}
