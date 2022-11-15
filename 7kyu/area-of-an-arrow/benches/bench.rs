#![no_std]
#![feature(test)]

extern crate test;
use area_of_an_arrow::arrow_area;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(arrow_area(black_box(25), black_box(25)));
        }
    });
}