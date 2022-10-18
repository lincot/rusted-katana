#![no_std]
#![feature(test)]

extern crate test;
use descending_order::descending_order;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| descending_order(black_box(1_254_859_723)));
}
