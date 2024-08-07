#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use to_square_root_or_not_to_square_root::square_or_square_root;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| square_or_square_root(black_box(&[100, 101, 5, 5, 1, 1])));
}
