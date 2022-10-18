#![no_std]
#![feature(test)]

extern crate test;
use find_the_divisors::divisors;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| divisors(black_box(1_604_677_850)));
}
