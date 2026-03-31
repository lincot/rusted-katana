#![feature(test)]

extern crate test;
use core::array;

use penultimate::penultimate;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a: [_; 16] = array::from_fn(|i| i as i32);
    bencher.iter(|| penultimate(black_box(&a)));
}
