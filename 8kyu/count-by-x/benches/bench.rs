#![no_std]
#![feature(test)]

extern crate test;
use count_by_x::count_by;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_by(black_box(5), black_box(100_000)));
}
