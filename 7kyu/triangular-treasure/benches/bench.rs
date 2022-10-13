#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use triangular_treasure::triangular;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(triangular(n));
        }
    });
}
