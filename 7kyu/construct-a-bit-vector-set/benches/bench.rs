#![feature(test)]

extern crate test;
use construct_a_bit_vector_set::sort_by_bit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sort_by_bit(black_box(&[1, 5, 30, 6, 10])));
}
