#![no_std]
#![feature(test)]

extern crate test;
use consecutive_letters::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| black_box(solve(black_box("abcdefghijklmnopqrstuvwxyz"))));
}
