#![feature(test)]

extern crate test;
use count_odd_numbers_below_n::odd_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| odd_count(black_box(1111)));
}
