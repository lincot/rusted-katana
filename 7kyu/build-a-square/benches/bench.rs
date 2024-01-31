#![feature(test)]

extern crate test;
use build_a_square::generate_shape;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| generate_shape(black_box(if cfg!(miri) { 10 } else { 1000 })));
}
