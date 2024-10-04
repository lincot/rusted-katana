#![feature(test)]

extern crate test;
use find_the_number_of_trailing_zeros_in_the_binary_representation_of_a_number::trailing_zeros;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| trailing_zeros(black_box(32)));
}
