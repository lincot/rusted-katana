#![feature(test)]

extern crate test;
use get_the_sum_of_multiples_of_triangular_numbers::sum_mult_triangnum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sum_mult_triangnum(black_box(100), black_box(100)));
}
