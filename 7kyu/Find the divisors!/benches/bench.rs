#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[allow(unused_must_use)]
#[bench]
fn bench(bencher: &mut Bencher) {
    let integer = black_box(1604677850);
    bencher.iter(|| solution::divisors(integer))
}
