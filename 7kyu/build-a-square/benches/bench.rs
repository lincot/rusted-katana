#![feature(test)]

extern crate test;
use build_a_square::generate_shape;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| generate_shape(n));
}
