#![no_std]
#![feature(test)]

extern crate test;
use disease_spread::epidemic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(epidemic(
            black_box(18),
            black_box(432),
            black_box(1004),
            black_box(1),
            black_box(0.00209),
            black_box(0.51),
        ))
    });
}
