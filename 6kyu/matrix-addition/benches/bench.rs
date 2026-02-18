#![feature(test)]

extern crate test;
use core::array;
use matrix_addition::matrix_addition;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const N: usize = if cfg!(miri) { 8 } else { 64 };
    let a: [_; N] = array::from_fn(|i| (i..i + N).map(|x| x as i32).collect::<Vec<_>>());
    let b: [_; N] = array::from_fn(|i| (i + 1..i + 1 + N).map(|x| x as i32).collect::<Vec<_>>());
    bencher.iter(|| matrix_addition(black_box(&a), black_box(&b)));
}
