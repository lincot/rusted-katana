#![no_std]
#![feature(test)]

extern crate test;
use simple_nearest_prime::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(35000)));
}
