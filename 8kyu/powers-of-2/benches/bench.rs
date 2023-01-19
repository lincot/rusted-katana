#![no_std]
#![feature(test)]

extern crate test;
use powers_of_2::powers_of_two;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| powers_of_two(black_box(100)));
}
