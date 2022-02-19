#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 5;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);

    bencher.iter(|| solution::multi_table(n))
}
