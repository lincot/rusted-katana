#![feature(test)]

extern crate test;
use collinearity::collinearity;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| collinearity(black_box(1), black_box(2), black_box(2), black_box(4)));
}
