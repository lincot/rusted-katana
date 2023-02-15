#![no_std]
#![feature(test)]

extern crate test;
use round_by_0_dot_5_steps::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution(black_box(475.2)));
            black_box(solution(black_box(475.6)));
            black_box(solution(black_box(475.8)));
        }
    });
}
