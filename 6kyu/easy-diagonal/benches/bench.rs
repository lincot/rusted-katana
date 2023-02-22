#![no_std]
#![feature(test)]

extern crate test;
use easy_diagonal::diagonal;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| diagonal(black_box(55), black_box(20)));
}
