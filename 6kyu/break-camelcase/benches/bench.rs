#![feature(test)]

extern crate test;
use break_camelcase::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(solution(black_box("верблюдСлучайИспытание")));
        }
    });
}
