#![feature(test)]

extern crate test;
use core::array;
use sums_of_parts::parts_sums;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let ls: [_; 1000] = array::from_fn(|i| 1337 * i as u64);
    bencher.iter(|| parts_sums(black_box(&ls)));
}
