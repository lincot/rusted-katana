#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 56239814;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);

    bencher.iter(|| solution::balanced_num(n))
}
