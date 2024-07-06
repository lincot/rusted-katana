#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use triangular_treasure::triangular;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| triangular(black_box(1000)));
}
