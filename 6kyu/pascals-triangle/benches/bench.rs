#![no_std]
#![feature(test)]

extern crate test;
use pascals_triangle::pascals_triangle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pascals_triangle(black_box(40)));
}
