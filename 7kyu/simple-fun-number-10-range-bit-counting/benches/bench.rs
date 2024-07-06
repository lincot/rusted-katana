#![feature(test)]

extern crate test;
use simple_fun_number_10_range_bit_counting::range_bit_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| range_bit_count(black_box(10), black_box(100)));
}
