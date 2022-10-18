#![no_std]
#![feature(test)]

extern crate test;
use how_much::how_much;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| how_much(black_box(10000), black_box(9900)));
}
