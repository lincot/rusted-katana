#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 5;

#[bench]
fn bench(b: &mut Bencher) {
    let n = black_box(N);

    b.iter(|| solution::multi_table(n))
}
