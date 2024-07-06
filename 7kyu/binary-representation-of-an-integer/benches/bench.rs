#![feature(test)]

extern crate test;
use binary_representation_of_an_integer::show_bits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| show_bits(black_box(-12336)));
}
