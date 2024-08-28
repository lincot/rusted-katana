#![feature(test)]

extern crate test;
use integer_depth::compute_depth;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| compute_depth(black_box(42)));
}
