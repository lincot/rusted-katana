#![feature(test)]

extern crate test;
use generate_range_of_integers::generate_range;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| generate_range(black_box(1000), black_box(2000), black_box(9)));
}
