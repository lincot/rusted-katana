#![feature(test)]

extern crate test;
use compute_element_in_aitkens_array::aitken;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| aitken(black_box(10), black_box(10)));
}
