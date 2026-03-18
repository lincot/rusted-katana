#![feature(test)]

extern crate test;
use string_ends_with::solution;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("strawberry"), black_box("berry")));
}
