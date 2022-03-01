#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let m = black_box(10000);
    let n = black_box(9900);
    bencher.iter(|| solution::how_much(m, n))
}
