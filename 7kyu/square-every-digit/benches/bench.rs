#![no_std]
#![feature(test)]

extern crate test;
use square_every_digit::square_digits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(square_digits(black_box(123_456_789)));
        }
    });
}
