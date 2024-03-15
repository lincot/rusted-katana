#![feature(test)]

extern crate test;
use pillars::pillars;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(pillars(black_box(10), black_box(10), black_box(10)));
        }
    });
}
