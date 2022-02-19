#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: i32 = 545;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);

    bencher.iter(|| solution::sum_of_n(n))
}
