#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 896219342;

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(N);

    bencher.iter(|| solution::max_rot(num))
}
