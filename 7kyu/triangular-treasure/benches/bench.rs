#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use triangular_treasure::triangular;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(triangular(black_box(1000)));
        }
    });
}
