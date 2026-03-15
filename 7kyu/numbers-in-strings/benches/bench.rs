#![feature(test)]

extern crate test;
use numbers_in_strings::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("gh12cdy695m1")));
}
