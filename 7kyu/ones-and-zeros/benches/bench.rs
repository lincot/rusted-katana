#![feature(test)]

extern crate test;
use ones_and_zeros::binary_slice_to_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| binary_slice_to_number(black_box(&[1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1])));
}
