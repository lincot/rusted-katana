#![no_std]
#![feature(test)]

extern crate test;
use robinson_crusoe::crusoe;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        crusoe(
            black_box(45),
            black_box(0.1),
            black_box(3),
            black_box(1.01),
            black_box(1.1),
        )
    });
}
