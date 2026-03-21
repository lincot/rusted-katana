#![feature(test)]

extern crate test;
use pillars::pillars;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pillars(black_box(1), black_box(10), black_box(10)));
}
