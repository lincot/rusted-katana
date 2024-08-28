#![feature(test)]

extern crate test;
use floating_point_approximation_i::f;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| f(black_box(1e-15)));
}
