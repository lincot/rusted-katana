#![no_std]
#![feature(test)]

extern crate test;
use area_of_an_arrow::arrow_area;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(25);
    let b = black_box(25);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(arrow_area(a, b));
        }
    });
}
