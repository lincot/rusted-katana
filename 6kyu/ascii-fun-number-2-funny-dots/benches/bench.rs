#![feature(test)]

extern crate test;
use ascii_fun_number_2_funny_dots::dot;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dot(black_box(10), black_box(10)));
}
