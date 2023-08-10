#![no_std]
#![feature(test)]

extern crate test;
use ieee_754_floating_point_numbers::{f32_to_ieee_754, f64_to_ieee_754};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(f32_to_ieee_754(black_box(15.875)));
            black_box(f64_to_ieee_754(black_box(15.875)));
        }
    });
}
