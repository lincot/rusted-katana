#![feature(test)]

extern crate test;
use binary_addition::add_binary;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(add_binary(black_box(37), black_box(345)));
        }
    });
}
