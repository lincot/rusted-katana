//! <https://www.codewars.com/kata/57f609022f4d534f05000024/train/rust>

#![feature(portable_simd)]

use std::simd::prelude::*;

pub fn stray(arr: &[u32]) -> u32 {
    let &[first, second, third, ..] = arr else {
        panic!();
    };
    if first != second {
        return if first == third { second } else { first };
    }

    let chunks = arr[2..].chunks_exact(16);
    let rem = chunks.remainder();

    for chunk in chunks {
        let mask = u32x16::from_slice(chunk)
            .simd_ne(u32x16::splat(first))
            .to_bitmask();
        if mask != 0 {
            return chunk[mask.trailing_zeros() as usize];
        }
    }

    rem.iter().copied().find(|&x| x != first).unwrap()
}
