#![no_std]
#![feature(test)]

extern crate test;
use count_by_x::count_by;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let x = black_box(5);
    let n = black_box(100_000);
    bencher.iter(|| count_by(x, n));
}
