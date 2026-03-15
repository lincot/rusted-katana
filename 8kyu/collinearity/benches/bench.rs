#![feature(test)]

extern crate test;
use collinearity::collinearity;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| collinearity(black_box(1), black_box(2), black_box(2), black_box(4)));
}
