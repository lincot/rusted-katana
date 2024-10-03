#![feature(test)]

extern crate test;
use break_camelcase::solution;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("camelCasingTesting")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box("верблюдСлучайТест")));
}
