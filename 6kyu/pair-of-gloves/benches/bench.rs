#![feature(test)]

extern crate test;
use pair_of_gloves::number_of_pairs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(number_of_pairs(black_box(&[
            "gray", "black", "purple", "purple", "gray", "black",
        ])));
        black_box(number_of_pairs(black_box(&[
            "red", "green", "blue", "blue", "red", "green", "red", "red", "red",
        ])));
    });
}
