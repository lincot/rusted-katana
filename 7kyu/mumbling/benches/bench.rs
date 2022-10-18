#![no_std]
#![feature(test)]

extern crate test;
use mumbling::accum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| accum(black_box("ZpglnRxqenU")));
}
