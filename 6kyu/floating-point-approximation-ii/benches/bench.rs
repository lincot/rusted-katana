#![feature(test)]

extern crate test;
use floating_point_approximation_ii::interp;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        interp(
            black_box(f64::sin),
            black_box(-10.),
            black_box(10.),
            black_box(100),
        )
    });
}
