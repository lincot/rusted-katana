#![feature(test)]

extern crate test;
use basics_08_find_next_higher_number_with_same_bits_1_s::next_higher;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| next_higher(black_box(5_000_000)));
}
