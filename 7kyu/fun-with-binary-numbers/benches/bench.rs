#![feature(test)]

extern crate test;
use fun_with_binary_numbers::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box(6), black_box(8)));
}
