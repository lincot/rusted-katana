//! <https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use vqsort::VqSort;

pub fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut cubes = cubes.to_vec();
    if dir == 'R' {
        VqSort::sort_ascending(&mut cubes);
    } else {
        VqSort::sort_descending(&mut cubes);
    }
    cubes
}
