#![no_std]
#![feature(test)]

extern crate test;
use incrementer::incrementer;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let nums = black_box(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 8]);
    bencher.iter(|| incrementer(nums));
}
