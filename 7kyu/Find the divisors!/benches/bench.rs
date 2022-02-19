#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const INTEGER: u32 = 1604677850;

#[allow(unused_must_use)]
#[bench]
fn bench(bencher: &mut Bencher) {
    let integer = black_box(INTEGER);

    bencher.iter(|| solution::divisors(integer))
}
