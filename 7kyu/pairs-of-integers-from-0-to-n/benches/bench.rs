#![feature(test)]

extern crate test;
use pairs_of_integers_from_0_to_n::generate_pairs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| generate_pairs(black_box(100)));
}
