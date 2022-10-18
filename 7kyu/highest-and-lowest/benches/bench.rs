#![no_std]
#![feature(test)]

extern crate test;
use highest_and_lowest::high_and_low;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| high_and_low(black_box("8 3 -5 42 -1 0 0 -9 4 7 4 -4")));
}
