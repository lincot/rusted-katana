#![feature(test)]

extern crate test;
use find_the_divisors::divisors;
use test::{black_box, Bencher};

#[allow(unused_must_use)]
#[bench]
fn bench(bencher: &mut Bencher) {
    let integer = black_box(1_604_677_850);
    bencher.iter(|| divisors(integer));
}
