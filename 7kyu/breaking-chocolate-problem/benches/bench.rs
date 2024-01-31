#![feature(test)]

extern crate test;
use breaking_chocolate_problem::break_chocolate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(break_chocolate(black_box(7), black_box(4)));
        }
    });
}
