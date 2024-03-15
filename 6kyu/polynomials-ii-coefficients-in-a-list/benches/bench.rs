#![feature(test)]

extern crate test;
use polynomials_ii_coefficients_in_a_list::calc_poly;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| calc_poly(black_box(&[2, 0, 5, -6, 4, 0]), black_box(2)));
}
