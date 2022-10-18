#![no_std]
#![feature(test)]

extern crate test;
use alternate_capitalization::capitalize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| capitalize(black_box("абракадабра")));
}
