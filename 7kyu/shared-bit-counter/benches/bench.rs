#![feature(test)]

extern crate test;
use shared_bit_counter::shared_bits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| shared_bits(black_box(43), black_box(77)));
}
