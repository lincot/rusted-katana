#![feature(test)]

extern crate test;
use quadratic_coefficients_solver::quadratic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| quadratic(black_box(100), black_box(200)));
}
