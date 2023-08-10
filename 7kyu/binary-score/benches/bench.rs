#![no_std]
#![feature(test)]

extern crate test;
use binary_score::score;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(score(black_box(2_376_499_510)));
        }
    });
}
