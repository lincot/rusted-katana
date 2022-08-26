#![feature(test)]

extern crate test;
use break_camelcase::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("верблюдСлучайИспытание");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution(s));
        }
    });
}
