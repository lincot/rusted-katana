#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use find_the_first_non_consecutive_number::first_non_consecutive;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr: [_; 1024] = array::from_fn(|i| {
        if i < 512 {
            1337 + i as i32
        } else {
            1337 * i as i32
        }
    });
    bencher.iter(|| first_non_consecutive(black_box(&arr)));
}
