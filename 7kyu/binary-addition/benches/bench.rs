#![feature(test)]

extern crate test;
use binary_addition::add_binary;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| add_binary(black_box(37), black_box(345)));
}
