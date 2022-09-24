#![no_std]
#![feature(test)]

extern crate test;
use calculate_bmi::bmi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let weight = black_box(60);
    let height = black_box(1.70);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(bmi(weight, height));
        }
    });
}
