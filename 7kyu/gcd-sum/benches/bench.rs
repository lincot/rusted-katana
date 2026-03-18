#![feature(test)]

extern crate test;
use gcd_sum::solve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(1000), black_box(5)));
}
