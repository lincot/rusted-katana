#![no_std]
#![feature(test)]

extern crate test;
use binary_addition::add_binary;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(37);
    let b = black_box(345);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(add_binary(a, b));
        }
    });
}
