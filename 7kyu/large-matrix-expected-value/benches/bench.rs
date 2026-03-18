#![feature(test)]

extern crate test;
use large_matrix_expected_value::expected_value;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| expected_value(black_box(5), black_box(11)));
}
