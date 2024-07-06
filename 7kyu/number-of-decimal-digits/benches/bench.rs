#![feature(test)]

extern crate test;
use number_of_decimal_digits::digits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| digits(black_box(16_146_655_587_590_542_430)));
}
