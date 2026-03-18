#![feature(test)]

extern crate test;
use multiply::multiply;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| multiply(black_box(-5), black_box(20)));
}
