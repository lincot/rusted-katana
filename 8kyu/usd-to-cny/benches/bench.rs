#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use usd_to_cny::usdcny;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| usdcny(black_box(465)));
}
