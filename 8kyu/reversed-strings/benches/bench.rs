#![no_std]
#![feature(test)]

extern crate test;
use reversed_strings::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("рандомизатор")));
}
