#![feature(test)]

extern crate test;
use safecracker::safecracker;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(safecracker(black_box(0), black_box(&(3, 10, 5))));
        black_box(safecracker(black_box(81), black_box(&(272, 244, 200))));
        black_box(safecracker(black_box(56), black_box(&(777, 722, 943))));
    });
}
