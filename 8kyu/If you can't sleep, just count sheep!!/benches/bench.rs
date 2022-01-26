#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u32 = 1000;

#[bench]
fn bench(b: &mut Bencher) {
    let n = black_box(N);

    b.iter(|| solution::count_sheep(n))
}
