//! <https://www.codewars.com/kata/5ff2093d375dca00170057bc/train/rust>

#![feature(portable_simd)]
use std::simd::{Select, prelude::*};

const LANES: usize = 32;

pub fn count_min_rotations(dice: &[u8]) -> u32 {
    let one = Simd::<u8, LANES>::splat(1);
    let zero = Simd::<u8, LANES>::splat(0);

    let mut counts = [0u32; 6];

    let (chunks, rest) = dice.as_chunks::<LANES>();
    for block in chunks.chunks(u8::MAX as _) {
        let mut acc = [Simd::<u8, LANES>::splat(0); 6];
        for chunk in block {
            let v = Simd::<u8, LANES>::from_array(*chunk);
            for f in 0..6u8 {
                acc[f as usize] += v.simd_eq(Simd::splat(f + 1)).select(one, zero);
            }
        }
        for f in 0..6 {
            counts[f] += acc[f].cast::<u16>().reduce_sum() as u32;
        }
    }

    for dice in rest {
        counts[(dice - 1) as usize] += 1;
    }

    (0..6)
        .map(|t| dice.len() as u32 - counts[t] + counts[5 - t])
        .min()
        .unwrap()
}
