#![feature(test)]

extern crate test;
use square_every_digit::square_digits;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| square_digits(black_box(123_456_789)));
}
