#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use merging_vectors_without_reallocation::merge;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ys = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    bencher.iter(|| merge(black_box(&xs), black_box(&ys)));
}
