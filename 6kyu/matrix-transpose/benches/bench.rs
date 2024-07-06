#![feature(test)]

extern crate test;
use core::array;
use matrix_transpose::transpose;
use test::{black_box, Bencher};

#[bench]
#[cfg(not(miri))]
fn bench_small(bencher: &mut Bencher) {
    let matrix: [_; 20] = array::from_fn(|_| (0..=20).collect::<Vec<_>>());
    bencher.iter(|| transpose(black_box(&matrix)));
}

#[bench]
#[cfg(not(miri))]
fn bench_medium(bencher: &mut Bencher) {
    let matrix: [_; 256] = array::from_fn(|_| (0..=255).collect::<Vec<_>>());
    bencher.iter(|| transpose(black_box(&matrix)));
}

#[bench]
#[cfg(not(miri))]
fn bench_big(bencher: &mut Bencher) {
    let matrix: [_; 20_000] = array::from_fn(|_| (0..=255).collect::<Vec<_>>());
    bencher.iter(|| transpose(black_box(&matrix)));
}
