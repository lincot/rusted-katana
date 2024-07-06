#![feature(test)]

extern crate test;
use fast_cooking_pancakes::cook_pancakes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cook_pancakes(black_box(782), black_box(261)));
}
