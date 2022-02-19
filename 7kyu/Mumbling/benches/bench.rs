#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("ZpglnRxqenU");
    bencher.iter(|| solution::accum(s))
}
