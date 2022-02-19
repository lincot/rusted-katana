#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(1254859723);
    bencher.iter(|| solution::descending_order(n))
}
