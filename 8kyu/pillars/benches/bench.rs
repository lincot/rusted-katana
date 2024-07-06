#![feature(test)]

extern crate test;
use pillars::pillars;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pillars(black_box(10), black_box(10), black_box(10)));
}
