#![no_std]
#![feature(test)]

extern crate test;
use take_the_derivative::derive;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let coefficient = black_box(20);
    let exponent = black_box(30);
    bencher.iter(|| derive(coefficient, exponent));
}
