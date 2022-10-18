#![no_std]
#![feature(test)]

extern crate test;
use filling_an_array_part_1::arr;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| arr(black_box(100_000)));
}
