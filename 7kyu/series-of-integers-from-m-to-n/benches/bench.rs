#![no_std]
#![feature(test)]

extern crate test;
use series_of_integers_from_m_to_n::integer_series;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| integer_series(black_box(700), black_box(10000)));
}
