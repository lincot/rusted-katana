#![feature(test)]

extern crate test;
use reversed_strings::solution;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("randomizer")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("рандомизатор")));
}
