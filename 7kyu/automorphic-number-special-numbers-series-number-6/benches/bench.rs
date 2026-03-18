#![feature(test)]

extern crate test;
use automorphic_number_special_numbers_series_number_6::automorphic;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| automorphic(black_box(625)));
}
