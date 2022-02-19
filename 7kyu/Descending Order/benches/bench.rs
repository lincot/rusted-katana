#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 1254859723;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);

    bencher.iter(|| solution::descending_order(n))
}
