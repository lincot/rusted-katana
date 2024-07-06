#![feature(test)]

extern crate test;
use calculate_bmi::bmi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| bmi(black_box(60), black_box(1.70)));
}
