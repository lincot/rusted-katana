#![feature(test)]

extern crate test;
use core::array;
use merging_vectors_without_reallocation::merge;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = array::from_fn::<_, 1000, _>(|i| 1337 * i).to_vec();
    let ys = array::from_fn::<_, 2000, _>(|i| 7331 * i).to_vec();
    bencher.iter(|| merge(black_box(&xs), black_box(&ys)));
}
