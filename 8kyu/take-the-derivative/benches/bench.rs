#![no_std]
#![feature(test)]

extern crate test;
use take_the_derivative::derive;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| derive(black_box(20), black_box(30)));
}
