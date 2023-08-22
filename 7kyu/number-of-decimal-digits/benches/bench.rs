#![no_std]
#![feature(test)]

extern crate test;
use number_of_decimal_digits::digits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(digits(black_box(16_146_655_587_590_542_430)));
        }
    });
}
