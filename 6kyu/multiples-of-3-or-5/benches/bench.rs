#![no_std]
#![feature(test)]

extern crate test;
use multiples_of_3_or_5::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(solution(black_box(45643)));
        }
    });
}
