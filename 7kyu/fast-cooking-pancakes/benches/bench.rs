#![feature(test)]

extern crate test;
use fast_cooking_pancakes::cook_pancakes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(cook_pancakes(black_box(782), black_box(261)));
        }
    });
}
