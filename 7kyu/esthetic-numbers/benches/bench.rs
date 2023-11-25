#![no_std]
#![feature(test)]

extern crate test;
use esthetic_numbers::esthetic;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| esthetic(black_box(528_132_674)));
}
