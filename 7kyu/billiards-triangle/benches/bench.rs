#![no_std]
#![feature(test)]

extern crate test;
use billiards_triangle::pyramid;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(pyramid(black_box(9999)));
        }
    });
}
