#![no_std]
#![feature(test)]

extern crate test;
use sixteen_circles::sixteen_circles;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(sixteen_circles(black_box(283)));
        }
    });
}
