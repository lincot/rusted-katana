#![feature(test)]

extern crate test;
use crack_the_pin::crack;
use test::{black_box, Bencher};

#[bench]
#[cfg(not(miri))]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| crack(black_box("dcddb75469b4b4875094e14561e573d8")));
}
