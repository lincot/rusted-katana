#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let phrase = black_box("рандомизатор");
    bencher.iter(|| solution::solution(phrase));
}
