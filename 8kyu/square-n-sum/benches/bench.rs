#![feature(test)]

extern crate test;
use square_n_sum::square_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let vec: Vec<_> = (0..if cfg!(miri) { 16 } else { 2048 })
        .map(|i| i as _)
        .collect();
    bencher.iter(|| square_sum(black_box(vec.clone())));
}
