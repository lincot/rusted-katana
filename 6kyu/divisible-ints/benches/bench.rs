#![feature(test)]

extern crate test;
use divisible_ints::get_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_count(black_box(1_121_111_112)));
}
