#![feature(test)]

extern crate test;
use string_ends_with::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("strawberry"), black_box("berry")));
}
