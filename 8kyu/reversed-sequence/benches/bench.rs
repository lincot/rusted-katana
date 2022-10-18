#![no_std]
#![feature(test)]

extern crate test;
use reversed_sequence::reverse_seq;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| reverse_seq(black_box(300_000)));
}
