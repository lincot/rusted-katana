#![no_std]
#![feature(test)]

extern crate test;
use nth_order_summation::s;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| s(black_box(100), black_box(250_000u32.into())));
}
