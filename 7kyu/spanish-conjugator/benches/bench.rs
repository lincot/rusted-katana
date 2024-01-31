#![feature(test)]

extern crate test;
use spanish_conjugator::conjugate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| conjugate(black_box("vivir")));
}
