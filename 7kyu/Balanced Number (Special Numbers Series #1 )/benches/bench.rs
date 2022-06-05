#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(56_239_814);
    bencher.iter(|| solution::balanced_num(n));
}
