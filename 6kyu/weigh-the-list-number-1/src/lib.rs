//! <https://www.codewars.com/kata/5fad2310ff1ef6003291a951/train/rust>

#![no_std]
#![feature(array_chunks)]

extern crate alloc;
use alloc::vec::Vec;

pub fn weigh_the_list(mut a: Vec<i64>) -> Vec<i64> {
    for [a, b] in a.array_chunks_mut() {
        (*a, *b) = (*b, -*a);
    }
    a
}
