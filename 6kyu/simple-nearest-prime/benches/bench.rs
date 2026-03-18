#![feature(test)]

extern crate test;
use simple_nearest_prime::solve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(35_000_000)));
}
