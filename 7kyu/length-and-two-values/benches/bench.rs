#![no_std]
#![feature(test)]

extern crate test;
use length_and_two_values::alternate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| alternate(black_box(10000), black_box("blue"), black_box("red")));
}
