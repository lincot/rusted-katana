#![no_std]
#![feature(test)]

extern crate test;
use bishop_movement_checker::bishop;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 1..1000 {
            black_box(bishop(black_box("a1"), black_box("b5"), black_box(5)));
        }
    });
}
