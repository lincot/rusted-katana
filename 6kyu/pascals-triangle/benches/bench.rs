#![feature(test)]

extern crate test;
use pascals_triangle::pascals_triangle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pascals_triangle(black_box(30)));
}
