#![no_std]
#![feature(test)]

extern crate test;
use breaking_chocolate_problem::break_chocolate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(7);
    let m = black_box(4);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(break_chocolate(n, m));
        }
    });
}
