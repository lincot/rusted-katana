#![feature(test)]

extern crate test;
use merging_vectors_without_reallocation::merge;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let xs = black_box(&xs);
    let ys = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let ys = black_box(&ys);
    bencher.iter(|| merge(xs, ys));
}
