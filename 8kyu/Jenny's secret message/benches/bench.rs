#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_johny(bencher: &mut Bencher) {
    let input = black_box("Johnny");
    bencher.iter(|| solution::greet(input))
}

#[bench]
fn bench_susan(bencher: &mut Bencher) {
    let input = black_box("Susan");
    bencher.iter(|| solution::greet(input))
}
