//! <https://www.codewars.com/kata/5fad2310ff1ef6003291a951/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn weigh_the_list(mut a: Vec<i64>) -> Vec<i64> {
    for pair in a.chunks_exact_mut(2) {
        pair.copy_from_slice(&[pair[1], -pair[0]]);
    }
    a
}
