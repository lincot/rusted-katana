#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let g = black_box(8);
    let m = black_box(30000);
    let n = black_box(100000);
    bencher.iter(|| solution::step(g, m, n))
}
