#![feature(test)]

extern crate test;
use round_by_0_dot_5_steps::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box(475.2)));
}
