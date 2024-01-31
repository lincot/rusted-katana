#![feature(test)]

extern crate test;
use calculate_bmi::bmi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(bmi(black_box(60), black_box(1.70)));
        }
    });
}
