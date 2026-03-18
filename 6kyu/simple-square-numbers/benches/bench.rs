#![feature(test)]

extern crate test;
use simple_square_numbers::solve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(290_101)));
}
