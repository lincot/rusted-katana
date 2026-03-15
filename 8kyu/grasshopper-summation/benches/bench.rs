#![feature(test)]

extern crate test;
use grasshopper_summation::summation;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| summation(black_box(35)));
}
