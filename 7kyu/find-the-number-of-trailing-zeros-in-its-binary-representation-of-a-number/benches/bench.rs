#![feature(test)]

extern crate test;
use find_the_number_of_trailing_zeros_in_its_binary_representation_of_a_number::trailing_zeros;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| trailing_zeros(black_box(32)));
}
