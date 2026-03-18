#![feature(test)]

extern crate test;
use consecutive_letters::solve;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("abcdefghijklmnopqrstuvwxyz")));
}
