#![no_std]
#![feature(test)]

extern crate test;
use square_every_digit::square_digits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(123_456_789);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(square_digits(num));
        }
    });
}
