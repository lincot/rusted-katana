#![feature(test)]

extern crate test;
use casino_chips::solve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(&[7, 4, 10])));
}
