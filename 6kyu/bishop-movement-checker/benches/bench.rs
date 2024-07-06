#![feature(test)]

extern crate test;
use bishop_movement_checker::bishop;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| bishop(black_box("a1"), black_box("b5"), black_box(5)));
}
