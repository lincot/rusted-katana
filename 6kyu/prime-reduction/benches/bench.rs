#![no_std]
#![feature(test)]

extern crate test;
use prime_reduction::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(100), black_box(10000)));
}
