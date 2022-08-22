#![feature(test)]

extern crate test;
use fun_with_binary_numbers::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(11);
    let b = black_box(8);
    bencher.iter(|| solution(n, b));
}
