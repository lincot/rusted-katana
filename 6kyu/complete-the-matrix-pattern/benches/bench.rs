#![feature(test)]

extern crate test;
use complete_the_matrix_pattern::make_matrix;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| make_matrix(black_box(12345), black_box(31)));
}
