#![feature(test)]

extern crate test;
use recursion_101::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(8_796_203), black_box(7556)));
}
