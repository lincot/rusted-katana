#![feature(test)]

extern crate test;
use floating_point_approximation_iii::quadratic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| quadratic(black_box(7.), black_box(4e+13), black_box(8.)));
}
