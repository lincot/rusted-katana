#![no_std]
#![feature(test)]

extern crate test;
use bugs_life::shortest_distance;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(134.);
    let b = black_box(191.5);
    let c = black_box(45.5);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(shortest_distance(a, b, c));
        }
    });
}
