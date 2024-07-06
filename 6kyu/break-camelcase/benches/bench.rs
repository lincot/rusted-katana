#![feature(test)]

extern crate test;
use break_camelcase::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("верблюдСлучайИспытание")));
}
