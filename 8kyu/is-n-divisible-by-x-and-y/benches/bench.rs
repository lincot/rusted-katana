#![feature(test)]

extern crate test;
use is_n_divisible_by_x_and_y::is_divisible;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_divisible(black_box(126_126), black_box(126), black_box(2)));
}
