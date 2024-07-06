#![feature(test)]

extern crate test;
use multiples_of_3_or_5::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box(45643)));
}
