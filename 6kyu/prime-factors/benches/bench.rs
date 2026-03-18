#![feature(test)]

extern crate test;
use prime_factors::prime_factors;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| prime_factors(black_box(78_152_045)));
}
