#![feature(test)]

extern crate test;
use core::array;
use find_all_non_consecutive_numbers::all_non_consecutive;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|i| i as i32 + i as i32 / 100);
    bencher.iter(|| all_non_consecutive(black_box(&arr)));
}
