#![feature(test)]

extern crate test;
use ieee_754_floating_point_numbers::{f32_to_ieee_754, f64_to_ieee_754};
use test::{black_box, Bencher};

#[bench]
fn bench_f32(bencher: &mut Bencher) {
    bencher.iter(|| f32_to_ieee_754(black_box(15.875)));
}

#[bench]
fn bench_f64(bencher: &mut Bencher) {
    bencher.iter(|| f64_to_ieee_754(black_box(15.875)));
}
